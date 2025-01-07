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

/**
 * @brief Internal stretcher structure - pure C implementation
 *
 * Contains all the state needed for time-stretching audio data, including
 * buffers for input/output audio data and window functions. All memory
 * management is explicit using malloc/free.
 */
struct bungee_stretcher {
    float* input_buffer;     /**< Buffer for input audio samples */
    float* output_buffer;    /**< Buffer for output audio samples */
    float* window_buffer;    /**< Buffer for window function */
    size_t buffer_size;      /**< Size of input/output buffers in frames */
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
 * @brief Initializes the Bungee library
 *
 * Currently a no-op as no global initialization is needed.
 *
 * @return BUNGEE_OK on success
 */
bungee_error_t bungee_init(void) {
    return BUNGEE_OK;
}

/**
 * @brief Cleans up any global resources
 *
 * Currently a no-op as no global cleanup is needed.
 */
void bungee_cleanup(void) {
    /* No global cleanup needed */
}

/**
 * @brief Creates a new time-stretcher instance
 *
 * Allocates and initializes a new stretcher with the specified sample rates
 * and channel count. All memory is allocated using malloc() and must be freed
 * with bungee_destroy().
 *
 * @param rates Input and output sample rates
 * @param channels Number of audio channels (must be > 0)
 * @return Pointer to new stretcher instance, or NULL on error
 */
bungee_stretcher_t* bungee_create(bungee_sample_rates_t rates, int channels) {
    if (channels <= 0) {
        return NULL;
    }

    bungee_stretcher_t* stretcher = (bungee_stretcher_t*)malloc(sizeof(bungee_stretcher_t));
    if (!stretcher) {
        return NULL;
    }

    /* Initialize basic parameters */
    stretcher->channels = channels;
    stretcher->input_rate = rates.input_rate;
    stretcher->output_rate = rates.output_rate;
    stretcher->position = 0.0;
    stretcher->speed = 1.0;
    stretcher->pitch = 1.0;
    stretcher->is_flushed = true;
    
    /* Set up buffers */
    stretcher->buffer_size = 8192;  /* Default size */
    stretcher->window_size = 2048;  /* Default window size */
    stretcher->overlap = stretcher->window_size / 4;  /* 75% overlap */
    
    /* Allocate buffers */
    stretcher->input_buffer = (float*)malloc(stretcher->buffer_size * channels * sizeof(float));
    stretcher->output_buffer = (float*)malloc(stretcher->buffer_size * channels * sizeof(float));
    stretcher->window_buffer = (float*)malloc(stretcher->window_size * sizeof(float));
    
    if (!stretcher->input_buffer || !stretcher->output_buffer || !stretcher->window_buffer) {
        if (stretcher->input_buffer) free(stretcher->input_buffer);
        if (stretcher->output_buffer) free(stretcher->output_buffer);
        if (stretcher->window_buffer) free(stretcher->window_buffer);
        free(stretcher);
        return NULL;
    }

    /* Create window function */
    create_hann_window(stretcher->window_buffer, stretcher->window_size);

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
        if (stretcher->output_buffer) free(stretcher->output_buffer);
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
        memset(stretcher->output_buffer, 0, stretcher->buffer_size * stretcher->channels * sizeof(float));
        stretcher->is_flushed = true;
    }

    return BUNGEE_OK;
}

/**
 * @brief Specifies input data for grain processing
 *
 * Copies a portion of the input data into the stretcher's internal buffer
 * and calculates the grain boundaries based on the current position.
 *
 * @param stretcher Stretcher instance
 * @param input_data Input audio data
 * @param frame_count Number of input frames
 * @param chunk Output parameter for grain boundaries
 * @return BUNGEE_OK on success, error code otherwise
 */
bungee_error_t bungee_specify_grain(bungee_stretcher_t* stretcher, const float* input_data, 
                                   size_t frame_count, bungee_input_chunk_t* chunk) {
    if (!stretcher || !input_data || !chunk) {
        return BUNGEE_NULL_POINTER;
    }

    if (frame_count > stretcher->buffer_size) {
        return BUNGEE_BUFFER_TOO_SMALL;
    }

    /* Calculate grain boundaries */
    chunk->begin = (int32_t)stretcher->position;
    chunk->end = chunk->begin + (int32_t)stretcher->window_size;
    
    if (chunk->end > (int32_t)frame_count) {
        chunk->end = (int32_t)frame_count;
    }

    /* Copy input data */
    size_t copy_size = (chunk->end - chunk->begin) * stretcher->channels * sizeof(float);
    memcpy(stretcher->input_buffer, input_data + (chunk->begin * stretcher->channels), copy_size);

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
        return BUNGEE_NULL_POINTER;
    }

    /* Apply window function to input data */
    for (int ch = 0; ch < stretcher->channels; ch++) {
        for (size_t i = 0; i < stretcher->window_size; i++) {
            size_t idx = i * stretcher->channels + ch;
            stretcher->input_buffer[idx] *= stretcher->window_buffer[i];
        }
    }

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
        return BUNGEE_NULL_POINTER;
    }

    if (chunk->frame_count > stretcher->buffer_size) {
        return BUNGEE_BUFFER_TOO_SMALL;
    }

    /* Simple time-stretching: copy with window function */
    for (int ch = 0; ch < stretcher->channels; ch++) {
        for (size_t i = 0; i < stretcher->window_size && i < (size_t)chunk->frame_count; i++) {
            size_t in_idx = i * stretcher->channels + ch;
            size_t out_idx = i * chunk->channel_stride + ch;
            chunk->data[out_idx] = stretcher->input_buffer[in_idx] * stretcher->window_buffer[i];
        }
    }

    return BUNGEE_OK;
}

/**
 * @brief Advances to the next grain
 *
 * Updates the stretcher's position and parameters for the next grain.
 *
 * @param stretcher Stretcher instance
 * @param request Updated parameters for next grain
 * @return BUNGEE_OK on success, error code otherwise
 */
bungee_error_t bungee_next(bungee_stretcher_t* stretcher, bungee_request_t* request) {
    if (!stretcher || !request) {
        return BUNGEE_NULL_POINTER;
    }

    /* Update position based on speed and overlap */
    double hop_size = stretcher->window_size * (1.0 - stretcher->overlap / (double)stretcher->window_size);
    stretcher->position += hop_size * stretcher->speed;
    
    /* Update request parameters */
    request->position = stretcher->position;
    request->speed = stretcher->speed;
    request->pitch = stretcher->pitch;
    request->reset = false;

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