#include "wrapper.h"
#include <iostream>

namespace bungee_rs {
    std::unique_ptr<Stretcher> new_stretcher(int32_t sample_rate, int32_t channels) {
        std::cout << "[C++] Creating new stretcher (sample_rate=" << sample_rate 
                  << ", channels=" << channels << ")" << std::endl;
        auto stretcher = std::make_unique<Stretcher>(
            std::pair<int32_t, int32_t>{sample_rate, sample_rate}, 
            channels
        );
        std::cout << "[C++] Stretcher created successfully" << std::endl;
        return stretcher;
    }

    void Stretcher::preroll(Request& request) const {
        std::cout << "[C++] preroll: Converting request (pitch=" << request.pitch 
                  << ", speed=" << request.speed 
                  << ", position=" << request.position << ")" << std::endl;
        
        Bungee::Request bungee_request{request.pitch, request.speed, request.position};
        Bungee::Stretcher<Bungee::Basic>::preroll(bungee_request);
        
        request.pitch = bungee_request.pitch;
        request.speed = bungee_request.speed;
        request.position = bungee_request.position;
        
        std::cout << "[C++] preroll: Updated request (pitch=" << request.pitch 
                  << ", speed=" << request.speed 
                  << ", position=" << request.position << ")" << std::endl;
    }

    InputChunk Stretcher::specify_grain(const Request& request) const {
        std::cout << "[C++] specify_grain: Converting request (pitch=" << request.pitch 
                  << ", speed=" << request.speed 
                  << ", position=" << request.position << ")" << std::endl;
        
        Bungee::Request bungee_request{request.pitch, request.speed, request.position};
        auto chunk = Bungee::Stretcher<Bungee::Basic>::specifyGrain(bungee_request);
        
        auto result = InputChunk{
            static_cast<int32_t>(chunk.offset),
            static_cast<int32_t>(chunk.length)
        };
        
        std::cout << "[C++] specify_grain: Returning chunk (offset=" << result.offset 
                  << ", length=" << result.length << ")" << std::endl;
        return result;
    }

    void Stretcher::analyse_grain(rust::Slice<const float> data, int32_t channel_stride) const {
        std::cout << "[C++] analyse_grain: Processing " << data.size() 
                  << " samples with stride " << channel_stride << std::endl;
        
        Bungee::Stretcher<Bungee::Basic>::analyseGrain(data.data(), channel_stride);
        
        std::cout << "[C++] analyse_grain: Analysis complete" << std::endl;
    }

    OutputChunk Stretcher::synthesise_grain() const {
        std::cout << "[C++] synthesise_grain: Starting synthesis" << std::endl;
        
        auto chunk = Bungee::Stretcher<Bungee::Basic>::synthesiseGrain();
        
        auto result = OutputChunk{
            std::vector<float>(chunk.data.begin(), chunk.data.end()),
            Request{chunk.startRequest.pitch, chunk.startRequest.speed, chunk.startRequest.position},
            Request{chunk.endRequest.pitch, chunk.endRequest.speed, chunk.endRequest.position}
        };
        
        std::cout << "[C++] synthesise_grain: Generated " << result.data.size() 
                  << " samples" << std::endl;
        std::cout << "[C++] synthesise_grain: Start request (pitch=" << result.start_request.pitch 
                  << ", speed=" << result.start_request.speed 
                  << ", position=" << result.start_request.position << ")" << std::endl;
        std::cout << "[C++] synthesise_grain: End request (pitch=" << result.end_request.pitch 
                  << ", speed=" << result.end_request.speed 
                  << ", position=" << result.end_request.position << ")" << std::endl;
        
        return result;
    }

    void Stretcher::next(Request& request) const {
        std::cout << "[C++] next: Converting request (pitch=" << request.pitch 
                  << ", speed=" << request.speed 
                  << ", position=" << request.position << ")" << std::endl;
        
        Bungee::Request bungee_request{request.pitch, request.speed, request.position};
        Bungee::Stretcher<Bungee::Basic>::next(bungee_request);
        
        request.pitch = bungee_request.pitch;
        request.speed = bungee_request.speed;
        request.position = bungee_request.position;
        
        std::cout << "[C++] next: Updated request (pitch=" << request.pitch 
                  << ", speed=" << request.speed 
                  << ", position=" << request.position << ")" << std::endl;
    }
} 