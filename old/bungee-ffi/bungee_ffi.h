#ifndef BUNGEE_FFI_H
#define BUNGEE_FFI_H

#include <stddef.h>
#include <stdbool.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

// Error codes
typedef enum {
    BUNGEE_OK = 0,
    BUNGEE_ERROR_NULL_POINTER,
    BUNGEE_ERROR_INVALID_PARAM,
    BUNGEE_ERROR_MEMORY,
    BUNGEE_ERROR_INVALID_STATE
} bungee_error_t;

// Stretcher state
typedef enum {
    BUNGEE_STATE_CREATED,
    BUNGEE_STATE_PREROLLED,
    BUNGEE_STATE_GRAIN_SPECIFIED,
    BUNGEE_STATE_GRAIN_ANALYSED,
    BUNGEE_STATE_GRAIN_SYNTHESISED
} bungee_state_t;

// Sample rates
typedef struct {
    float input_rate;
    float output_rate;
} bungee_sample_rates_t;

// Request parameters
typedef struct {
    float time_ratio;
    float pitch_scale;
    size_t window_size;
    size_t step_size;
} bungee_request_t;

// Opaque stretcher type
typedef struct {
    bungee_sample_rates_t rates;
    int channels;
    bungee_state_t state;
    // Additional implementation details hidden
} bungee_stretcher_t;

// Initialization and cleanup
bungee_error_t bungee_init(void);
void bungee_cleanup(void);

// Core stretcher functions
bungee_stretcher_t* bungee_create(bungee_sample_rates_t rates, int channels);
void bungee_destroy(bungee_stretcher_t* stretcher);

// Processing functions
bungee_error_t bungee_preroll(bungee_stretcher_t* stretcher, const bungee_request_t* request);
bungee_error_t bungee_specify_grain(bungee_stretcher_t* stretcher, const float* input_data, size_t frame_count);
bungee_error_t bungee_analyse_grain(bungee_stretcher_t* stretcher);
bungee_error_t bungee_synthesise_grain(bungee_stretcher_t* stretcher, float* output_data, size_t frame_count);
bungee_error_t bungee_next(bungee_stretcher_t* stretcher);

// Query functions
bool bungee_is_flushed(const bungee_stretcher_t* stretcher);
size_t bungee_max_input_frame_count(const bungee_stretcher_t* stretcher);

// Debug and monitoring functions
void bungee_print_memory_stats(void);
void bungee_set_debug_log(FILE* log_file);

#ifdef __cplusplus
}
#endif

#endif // BUNGEE_FFI_H 