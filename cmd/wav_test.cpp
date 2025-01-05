#include <bungee/bungee.hpp>
#include <iostream>
#include <vector>
#include <cmath>
#include <string>
#include <memory>
#include <sndfile.h>

// Helper function to convert semitones to pitch ratio
double semitones_to_pitch(double semitones) {
    return std::pow(2.0, semitones / 12.0);
}

int main(int argc, char* argv[]) {
    try {
        // Parameters
        const char* input_path = "../audio/Instrumental - Heart - 190.5bpm - Emaj.wav";
        const char* output_path = "../audio/cpp_output_pitched.wav";
        const double semitones = 12.0; // Pitch up one octave
        const bool full_duration = false;
        const double target_duration = full_duration ? 0.0 : 30.0; // Process 30s if not full duration

        // Open input file
        SF_INFO sf_info;
        std::cout << "Opening input file: " << input_path << std::endl;
        SNDFILE* input_file = sf_open(input_path, SFM_READ, &sf_info);
        if (!input_file) {
            throw std::runtime_error("Could not open input file");
        }

        // Read input samples
        std::vector<float> input_samples(sf_info.frames * sf_info.channels);
        sf_count_t frames_read = sf_readf_float(input_file, input_samples.data(), sf_info.frames);
        sf_close(input_file);

        std::cout << "Input specs:" << std::endl
                  << "  - Duration: " << static_cast<float>(sf_info.frames) / sf_info.samplerate << " seconds" << std::endl
                  << "  - Sample rate: " << sf_info.samplerate << " Hz" << std::endl
                  << "  - Channels: " << sf_info.channels << std::endl
                  << "  - Frames: " << sf_info.frames << std::endl;

        // Create stretcher
        auto stretcher = std::make_unique<bungee::Stretcher>(sf_info.samplerate, sf_info.channels);
        
        // Initialize request with pitch shift
        bungee::Request request;
        request.position = 0.0;
        request.speed = 1.0;
        request.pitch = semitones_to_pitch(semitones);
        request.reset = true;

        // Preroll
        stretcher->preroll(request);

        // Calculate target frames
        size_t target_frames = target_duration > 0.0 
            ? static_cast<size_t>(target_duration * sf_info.samplerate)
            : sf_info.frames;

        // Expected output duration
        double expected_duration = stretcher->calculate_output_duration(
            target_duration > 0.0 ? target_duration : static_cast<double>(sf_info.frames) / sf_info.samplerate,
            request
        );
        size_t expected_frames = stretcher->calculate_output_frames(target_frames, request);

        std::cout << "\nProcessing configuration:" << std::endl
                  << "  - Target duration: " << (target_duration > 0.0 ? target_duration : static_cast<double>(sf_info.frames) / sf_info.samplerate) << " seconds" << std::endl
                  << "  - Target frames: " << target_frames << std::endl
                  << "  - Expected output duration: " << expected_duration << " seconds" << std::endl
                  << "  - Expected output frames: " << expected_frames << std::endl
                  << "  - Full duration mode: " << std::boolalpha << full_duration << std::endl
                  << "  - Speed: " << request.speed << std::endl
                  << "  - Pitch: " << request.pitch << " (" << semitones << " semitones)" << std::endl;

        // Process audio
        std::vector<float> output_samples;
        size_t processed_frames = 0;
        size_t total_output_frames = 0;
        size_t chunk_count = 0;

        std::cout << "\nStarting audio processing..." << std::endl;

        while (processed_frames < sf_info.frames && processed_frames < target_frames) {
            chunk_count++;
            
            // Get next grain specification
            auto chunk = stretcher->specify_grain(request);
            
            // Prepare grain data with zero-padding for negative indices
            std::vector<float> grain_data;
            size_t start_idx = chunk.begin >= 0 ? chunk.begin * sf_info.channels : 0;
            size_t end_idx = chunk.end * sf_info.channels;
            size_t pad_start = chunk.begin < 0 ? -chunk.begin * sf_info.channels : 0;
            
            grain_data.resize((chunk.end - chunk.begin) * sf_info.channels, 0.0f);
            
            if (chunk.begin >= 0 && chunk.end > chunk.begin) {
                std::copy(
                    input_samples.begin() + start_idx,
                    input_samples.begin() + std::min(end_idx, input_samples.size()),
                    grain_data.begin() + pad_start
                );
            }
            
            // Analyze grain
            stretcher->analyse_grain(grain_data, sf_info.channels);
            
            // Prepare output chunk
            const size_t chunk_max_frames = 1024;
            std::vector<float> chunk_data(chunk_max_frames * sf_info.channels, 0.0f);
            bungee::OutputChunk output_chunk{
                chunk_data.data(),
                static_cast<int32_t>(chunk_max_frames),
                sf_info.channels,
                request,
                request
            };
            
            // Synthesize grain
            stretcher->synthesise_grain(output_chunk);
            
            // Collect output
            size_t output_frames = output_chunk.frame_count * sf_info.channels;
            if (output_frames > 0) {
                output_samples.insert(
                    output_samples.end(),
                    chunk_data.begin(),
                    chunk_data.begin() + output_frames
                );
                total_output_frames += output_chunk.frame_count;
            }
            
            // Advance to next chunk
            stretcher->next(request);
            
            // Update processed frames count
            size_t chunk_frames = chunk.end > 0 
                ? chunk.end - std::max(0, chunk.begin)
                : 0;
            processed_frames += chunk_frames;
            
            // Show progress
            if (processed_frames % (target_frames / 10) == 0) {
                std::cout << "Progress: " 
                          << (processed_frames * 100) / target_frames 
                          << "% (" << processed_frames << "/" << target_frames << " frames)" 
                          << std::endl;
            }
        }

        // Write output file
        std::cout << "\nProcessing complete:" << std::endl
                  << "  - Processed " << chunk_count << " chunks" << std::endl
                  << "  - Input frames processed: " << processed_frames << std::endl
                  << "  - Output frames generated: " << total_output_frames << std::endl
                  << "  - Output duration: " << static_cast<double>(total_output_frames) / sf_info.samplerate << " seconds" << std::endl
                  << "  - Target duration: " << target_duration << " seconds" << std::endl
                  << "  - Expected output duration: " << expected_duration << " seconds" << std::endl;

        std::cout << "\nWriting output file..." << std::endl;
        
        // Open output file
        SF_INFO output_info = sf_info;
        output_info.frames = total_output_frames;
        SNDFILE* output_file = sf_open(output_path, SFM_WRITE, &output_info);
        if (!output_file) {
            throw std::runtime_error("Could not open output file");
        }

        // Write output samples
        sf_count_t frames_written = sf_writef_float(output_file, output_samples.data(), total_output_frames);
        sf_close(output_file);

        std::cout << "Output written to: " << output_path << std::endl;

        // Validate duration
        if (std::abs(static_cast<double>(total_output_frames) / sf_info.samplerate - expected_duration) > 0.1) {
            std::cout << "\n❌ Duration validation failed!" << std::endl;
        } else {
            std::cout << "\n✅ Duration validation passed!" << std::endl;
        }

        return 0;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
} 