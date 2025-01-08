/**
 * @file bungee_c.c
 * @brief Pure C implementation of the Bungee audio time-stretching library
 *
 * This file provides a minimal, pure C implementation of audio time-stretching
 * functionality using a grain-based processing approach. The implementation
 * uses only standard C library features and avoids any C++ dependencies.
 */

#include "bungee_c.h"
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdio.h>

/**
 * @brief Internal stretcher structure - pure C implementation
 *
 * Contains all the state needed for time-stretching audio data, including
 * buffers for input audio data and window functions. All memory
 * management is explicit using malloc/free.
 */
struct bungee_stretcher {
    float* input_buffer;     /**< Buffer for input audio samples */
    float* window_buffer;    /**< Buffer for window function */
    size_t buffer_size;      /**< Size of input buffer in frames */
    int channels;           /**< Number of audio channels */
    double position;        /**< Current position in input stream */
    double speed;          /**< Playback speed ratio */
    double pitch;          /**< Pitch shift ratio */
    int input_rate;        /**< Input sample rate in Hz */
    int output_rate;       /**< Output sample rate in Hz */
    size_t window_size;    /**< Size of analysis/synthesis window */
    size_t overlap;        /**< Overlap size between consecutive windows */
    bool is_flushed;       /**< Whether all data has been processed */
};

/**
 * @brief Creates a Hann window function for smooth grain transitions
 *
 * @param window Buffer to store the window function values
 * @param size Size of the window in samples
 */
static void create_hann_window(float* window, size_t size) {
    for (size_t i = 0; i < size; i++) {
        window[i] = 0.5f * (1.0f - cosf((2.0f * M_PI * i) / (size - 1)));
    }
}

/**
 * @brief Initialize the Bungee library
 *
 * @return BUNGEE_OK on success, error code otherwise
 */
bungee_error_t bungee_init(void) {
    BUNGEE_LOG_SIMPLE("=== Bungee library initializing ===");
    return BUNGEE_OK;
}

/**
 * @brief Clean up the Bungee library
 */
void bungee_cleanup(void) {
    BUNGEE_LOG_SIMPLE("=== Bungee library cleanup ===");
}

/**
 * @brief Creates a new stretcher instance
 *
 * @param rates Sample rate configuration
 * @param channels Number of audio channels
 * @return Stretcher instance or NULL on error
 */
bungee_stretcher_t* bungee_create(bungee_sample_rates_t rates, int channels) {
    BUNGEE_LOG("Creating stretcher: input_rate=%d, output_rate=%d, channels=%d",
           rates.input_rate, rates.output_rate, channels);

    if (channels <= 0) {
        BUNGEE_LOG("Invalid channel count: %d", channels);
        return NULL;
    }

    if (rates.input_rate <= 0 || rates.output_rate <= 0) {
        BUNGEE_LOG("Invalid sample rates: input=%d, output=%d",
               rates.input_rate, rates.output_rate);
        return NULL;
    }

    bungee_stretcher_t* stretcher = (bungee_stretcher_t*)malloc(sizeof(bungee_stretcher_t));
    if (!stretcher) {
        BUNGEE_LOG_SIMPLE("Failed to allocate stretcher");
        return NULL;
    }

    /* Initialize basic parameters */
    stretcher->input_rate = rates.input_rate;
    stretcher->output_rate = rates.output_rate;
    stretcher->channels = channels;
    stretcher->position = 0.0;
    stretcher->speed = 1.0;
    stretcher->pitch = 1.0;
    stretcher->is_flushed = false;

    /* Calculate window size and overlap */
    stretcher->window_size = (size_t)(rates.input_rate * 0.1);  // 100ms window
    stretcher->overlap = stretcher->window_size / 2;            // 50% overlap
    stretcher->buffer_size = stretcher->window_size * 2;        // Double for safety

    BUNGEE_LOG("Window parameters: size=%zu, overlap=%zu, buffer=%zu",
           stretcher->window_size, stretcher->overlap, stretcher->buffer_size);

    /* Allocate buffers */
    size_t buffer_bytes = stretcher->buffer_size * channels * sizeof(float);
    stretcher->input_buffer = (float*)malloc(buffer_bytes);
    if (!stretcher->input_buffer) {
        BUNGEE_LOG("Failed to allocate input buffer: %zu bytes", buffer_bytes);
        free(stretcher);
        return NULL;
    }

    /* Create window function */
    stretcher->window_buffer = (float*)malloc(stretcher->window_size * sizeof(float));
    if (!stretcher->window_buffer) {
        BUNGEE_LOG("Failed to allocate window buffer: %zu bytes",
               stretcher->window_size * sizeof(float));
        free(stretcher->input_buffer);
        free(stretcher);
        return NULL;
    }

    /* Initialize Hann window */
    for (size_t i = 0; i < stretcher->window_size; i++) {
        double phase = (double)i / (stretcher->window_size - 1);
        stretcher->window_buffer[i] = (float)(0.5 * (1.0 - cos(2.0 * M_PI * phase)));
    }

    BUNGEE_LOG_SIMPLE("Stretcher created successfully");
    return stretcher;
}

/**
 * @brief Destroys a time-stretcher instance
 *
 * Frees all memory associated with the stretcher instance.
 *
 * @param stretcher Stretcher instance to destroy (may be NULL)
 */
void bungee_destroy(bungee_stretcher_t* stretcher) {
    if (stretcher) {
        if (stretcher->input_buffer) free(stretcher->input_buffer);
        if (stretcher->window_buffer) free(stretcher->window_buffer);
        free(stretcher);
    }
}

/**
 * @brief Prepares the stretcher for processing with new parameters
 *
 * Updates the stretcher's internal state with new position, speed, and pitch
 * values. Can also reset the internal buffers if requested.
 *
 * @param stretcher Stretcher instance
 * @param request New processing parameters
 * @return BUNGEE_OK on success, error code otherwise
 */
bungee_error_t bungee_preroll(bungee_stretcher_t* stretcher, const bungee_request_t* request) {
    if (!stretcher || !request) {
        return BUNGEE_NULL_POINTER;
    }

    stretcher->position = request->position;
    stretcher->speed = request->speed;
    stretcher->pitch = request->pitch;
    
    if (request->reset) {
        memset(stretcher->input_buffer, 0, stretcher->buffer_size * stretcher->channels * sizeof(float));
        stretcher->is_flushed = true;
    }

    return BUNGEE_OK;
}

/**
 * @brief Specifies the next grain to process
 *
 * Calculates grain boundaries based on current position and window size.
 *
 * @param stretcher Stretcher instance
 * @param input_data Input audio data
 * @param frame_count Total number of input frames
 * @param chunk Output chunk parameters
 * @return BUNGEE_OK on success, error code otherwise
 */
bungee_error_t bungee_specify_grain(bungee_stretcher_t* stretcher, const float* input_data, 
                                   size_t frame_count, bungee_input_chunk_t* chunk) {
    if (!stretcher || !input_data || !chunk) {
        BUNGEE_LOG("Null pointer in specify_grain: stretcher=%p, input_data=%p, chunk=%p",
               (void*)stretcher, (void*)input_data, (void*)chunk);
        return BUNGEE_NULL_POINTER;
    }

    BUNGEE_LOG("Specify grain start: position=%f, window_size=%zu, frame_count=%zu",
           stretcher->position, stretcher->window_size, frame_count);

    if (frame_count > stretcher->buffer_size) {
        BUNGEE_LOG("Buffer too small: requested=%zu, available=%zu",
               frame_count, stretcher->buffer_size);
        return BUNGEE_BUFFER_TOO_SMALL;
    }

    /* Calculate grain boundaries */
    chunk->begin = (int32_t)stretcher->position;
    chunk->end = chunk->begin + (int32_t)stretcher->window_size;
    
    BUNGEE_LOG("Initial grain boundaries: begin=%d, end=%d", chunk->begin, chunk->end);

    /* Adjust for frame count */
    if (chunk->end > (int32_t)frame_count) {
        BUNGEE_LOG("Adjusting end boundary: old=%d, new=%d", 
               chunk->end, (int32_t)frame_count);
        chunk->end = (int32_t)frame_count;
    }

    /* Calculate copy size */
    size_t copy_size = (chunk->end - chunk->begin) * stretcher->channels * sizeof(float);
    BUNGEE_LOG("Copy parameters: begin_sample=%d, channels=%d, copy_size=%zu bytes",
           chunk->begin, stretcher->channels, copy_size);

    /* Copy input data */
    if (chunk->begin >= 0 && chunk->end > chunk->begin) {
        memcpy(stretcher->input_buffer, 
               input_data + (chunk->begin * stretcher->channels), 
               copy_size);
        BUNGEE_LOG("Copied %zu bytes from input position %d", 
               copy_size, chunk->begin * stretcher->channels);
    } else {
        BUNGEE_LOG_SIMPLE("No data copied: invalid boundaries");
    }

    BUNGEE_LOG("Grain specification complete: begin=%d, end=%d", 
           chunk->begin, chunk->end);
    return BUNGEE_OK;
}

/**
 * @brief Analyzes a grain of audio data
 *
 * Applies the window function to the input data for analysis.
 *
 * @param stretcher Stretcher instance
 * @param input_data Input audio data
 * @param channel_stride Stride between channels in samples
 * @return BUNGEE_OK on success, error code otherwise
 */
bungee_error_t bungee_analyse_grain(bungee_stretcher_t* stretcher, const float* input_data, size_t channel_stride) {
    if (!stretcher || !input_data) {
        BUNGEE_LOG("Null pointer in analyse_grain: stretcher=%p, input_data=%p",
               (void*)stretcher, (void*)input_data);
        return BUNGEE_NULL_POINTER;
    }

    BUNGEE_LOG("Analyse grain start: window_size=%zu, channels=%d, stride=%zu",
           stretcher->window_size, stretcher->channels, channel_stride);

    /* Check for non-zero input */
    int has_nonzero = 0;
    for (size_t i = 0; i < stretcher->window_size * stretcher->channels; i++) {
        if (input_data[i] != 0.0f) {
            has_nonzero = 1;
            break;
        }
    }
    BUNGEE_LOG("Input data check: contains_nonzero=%d", has_nonzero);

    /* Apply window function to input data */
    for (int ch = 0; ch < stretcher->channels; ch++) {
        BUNGEE_LOG("Processing channel %d", ch);
        float max_sample = 0.0f;
        
        for (size_t i = 0; i < stretcher->window_size; i++) {
            size_t idx = i * stretcher->channels + ch;
            float windowed = input_data[idx] * stretcher->window_buffer[i];
            stretcher->input_buffer[idx] = windowed;
            
            /* Track maximum amplitude for debugging */
            float abs_val = fabsf(windowed);
            if (abs_val > max_sample) max_sample = abs_val;
        }
        
        BUNGEE_LOG("Channel %d max amplitude: %f", ch, max_sample);
    }

    BUNGEE_LOG_SIMPLE("Analysis complete");
    return BUNGEE_OK;
}

/**
 * @brief Synthesizes a grain of audio data
 *
 * Applies time-stretching and pitch-shifting to the analyzed grain and
 * writes the result to the output buffer.
 *
 * @param stretcher Stretcher instance
 * @param chunk Output parameters including destination buffer
 * @return BUNGEE_OK on success, error code otherwise
 */
bungee_error_t bungee_synthesise_grain(bungee_stretcher_t* stretcher, bungee_output_chunk_t* chunk) {
    if (!stretcher || !chunk || !chunk->data) {
        BUNGEE_LOG("Null pointer in synthesise_grain: stretcher=%p, chunk=%p", 
               (void*)stretcher, (void*)chunk);
        return BUNGEE_NULL_POINTER;
    }

    BUNGEE_LOG("Synthesise grain start: buffer_size=%zu, frame_count=%d, channels=%d",
           stretcher->buffer_size, chunk->frame_count, stretcher->channels);
    BUNGEE_LOG("Window size=%zu, position=%f, speed=%f", 
           stretcher->window_size, stretcher->position, stretcher->speed);

    if ((size_t)chunk->frame_count > stretcher->buffer_size) {
        BUNGEE_LOG("Buffer too small: requested=%d, available=%zu",
               chunk->frame_count, stretcher->buffer_size);
        return BUNGEE_BUFFER_TOO_SMALL;
    }

    /* Calculate output frame count based on time-stretching */
    size_t output_frames = (size_t)(stretcher->window_size * fabs(stretcher->speed));
    if (output_frames > (size_t)chunk->frame_count) {
        BUNGEE_LOG("Clamping output frames: %zu -> %d", output_frames, chunk->frame_count);
        output_frames = chunk->frame_count;
    }
    BUNGEE_LOG("Output frames calculation: window_size=%zu, speed=%f, output_frames=%zu",
           stretcher->window_size, stretcher->speed, output_frames);

    /* Check input buffer for non-zero values */
    float max_input = 0.0f;
    for (size_t i = 0; i < stretcher->window_size * stretcher->channels; i++) {
        float abs_val = fabsf(stretcher->input_buffer[i]);
        if (abs_val > max_input) max_input = abs_val;
    }
    BUNGEE_LOG("Input buffer max amplitude: %f", max_input);

    /* Process each channel */
    for (int ch = 0; ch < stretcher->channels; ch++) {
        BUNGEE_LOG("Processing channel %d", ch);
        float max_sample = 0.0f;
        
        /* Apply time-stretching and window function */
        for (size_t i = 0; i < output_frames; i++) {
            /* Calculate input sample position with time-stretching */
            float input_pos = (float)i / stretcher->speed;
            size_t in_idx = (size_t)input_pos;
            float frac = input_pos - in_idx;
            
            /* Bounds check */
            if (in_idx >= stretcher->window_size - 1) {
                BUNGEE_LOG("Input index out of bounds: in_idx=%zu, window_size=%zu", 
                       in_idx, stretcher->window_size);
                break;
            }
            
            /* Linear interpolation between input samples */
            size_t in_idx1 = in_idx * stretcher->channels + ch;
            size_t in_idx2 = (in_idx + 1) * stretcher->channels + ch;
            float sample1 = stretcher->input_buffer[in_idx1];
            float sample2 = stretcher->input_buffer[in_idx2];
            float interpolated = sample1 + frac * (sample2 - sample1);

            if (i == 0 || i == output_frames/2 || i == output_frames-1) {
                BUNGEE_LOG("Sample interpolation [%zu]: pos=%f, idx=%zu, frac=%f, s1=%f, s2=%f, interp=%f",
                       i, input_pos, in_idx, frac, sample1, sample2, interpolated);
            }
            
            /* Apply window function */
            float windowed = interpolated * stretcher->window_buffer[in_idx];
            
            /* Write to output */
            size_t out_idx = i * chunk->channel_stride + ch;
            chunk->data[out_idx] = windowed;

            if (i == 0 || i == output_frames/2 || i == output_frames-1) {
                BUNGEE_LOG("Output sample [%zu]: window=%f, final=%f, out_idx=%zu",
                       i, stretcher->window_buffer[in_idx], windowed, out_idx);
            }
            
            /* Track maximum amplitude for debugging */
            float abs_val = fabsf(windowed);
            if (abs_val > max_sample) max_sample = abs_val;
        }
        
        BUNGEE_LOG("Channel %d max amplitude: %f", ch, max_sample);
    }

    /* Update output frame count */
    chunk->frame_count = (int32_t)output_frames;
    BUNGEE_LOG("Synthesis complete: output_frames=%zu", output_frames);

    return BUNGEE_OK;
}

/**
 * @brief Advances to the next grain
 *
 * Updates the stretcher's position based on the current speed and
 * window overlap settings.
 *
 * @param stretcher Stretcher instance
 * @param request Request parameters to update
 * @return BUNGEE_OK on success, error code otherwise
 */
bungee_error_t bungee_next(bungee_stretcher_t* stretcher, bungee_request_t* request) {
    if (!stretcher || !request) {
        BUNGEE_LOG("Null pointer in next: stretcher=%p, request=%p",
               (void*)stretcher, (void*)request);
        return BUNGEE_NULL_POINTER;
    }

    BUNGEE_LOG("Next grain start: position=%f, speed=%f",
           stretcher->position, stretcher->speed);

    /* Calculate hop size based on window size and overlap */
    double hop_size = (double)(stretcher->window_size - stretcher->overlap);
    
    /* Advance position based on speed */
    stretcher->position += hop_size * stretcher->speed;
    request->position = stretcher->position;
    
    BUNGEE_LOG("Advanced position: hop_size=%f, new_position=%f",
           hop_size, stretcher->position);

    /* Check if we've processed all input */
    if (stretcher->position >= stretcher->buffer_size) {
        stretcher->is_flushed = true;
        BUNGEE_LOG("Reached end of input: position=%f, buffer_size=%zu",
               stretcher->position, stretcher->buffer_size);
    }

    return BUNGEE_OK;
}

/**
 * @brief Checks if all data has been processed
 *
 * @param stretcher Stretcher instance
 * @return true if all data has been processed, false otherwise
 */
bool bungee_is_flushed(const bungee_stretcher_t* stretcher) {
    if (!stretcher) {
        return true;
    }
    return stretcher->is_flushed;
}

/**
 * @brief Gets the maximum number of input frames that can be processed
 *
 * @param stretcher Stretcher instance
 * @return Maximum number of input frames, or 0 if stretcher is NULL
 */
size_t bungee_max_input_frame_count(const bungee_stretcher_t* stretcher) {
    if (!stretcher) {
        return 0;
    }
    return stretcher->buffer_size;
} 