#ifndef BUNGEE_FFI_H
#define BUNGEE_FFI_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>

// Opaque handle to the stretcher
typedef struct BungeeStretcher BungeeStretcher;

// Error codes
typedef enum {
    BUNGEE_OK = 0,
    BUNGEE_ERROR_NULL_POINTER = -1,
    BUNGEE_ERROR_INVALID_PARAM = -2,
    BUNGEE_ERROR_MEMORY = -3,
    BUNGEE_ERROR_PROCESSING = -4
} BungeeError;

// Configuration struct
typedef struct {
    uint32_t sample_rate;
    uint32_t channels;
    float time_ratio;    // 1.0 = normal speed
    float pitch_scale;   // 1.0 = normal pitch
} BungeeConfig;

// Create a new stretcher instance
BungeeStretcher* bungee_create(const BungeeConfig* config, BungeeError* error);

// Destroy a stretcher instance
void bungee_destroy(BungeeStretcher* stretcher);

// Process audio data
BungeeError bungee_process(
    BungeeStretcher* stretcher,
    const float* input_buffer,
    size_t input_frames,
    float* output_buffer,
    size_t output_frames,
    size_t* frames_used,
    size_t* frames_generated
);

// Flush any remaining samples
BungeeError bungee_flush(
    BungeeStretcher* stretcher,
    float* output_buffer,
    size_t output_frames,
    size_t* frames_generated
);

// Get required output buffer size for given input size
size_t bungee_get_required_output_size(
    const BungeeStretcher* stretcher,
    size_t input_frames
);

// Reset the stretcher state
BungeeError bungee_reset(BungeeStretcher* stretcher);

#ifdef __cplusplus
}
#endif

#endif // BUNGEE_FFI_H 