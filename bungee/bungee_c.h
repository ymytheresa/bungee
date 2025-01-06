#ifndef BUNGEE_C_H
#define BUNGEE_C_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdbool.h>

typedef enum {
    BUNGEE_OK = 0,
    BUNGEE_NULL_POINTER,
    BUNGEE_INVALID_PARAM,
    BUNGEE_MEMORY,
    BUNGEE_INVALID_STATE
} bungee_error_t;

typedef struct {
    float input_rate;
    float output_rate;
} bungee_sample_rates_t;

typedef struct {
    float time_ratio;
    float pitch_scale;
    size_t window_size;
    size_t step_size;
} bungee_request_t;

typedef struct bungee_stretcher_t bungee_stretcher_t;

// Core functions
bungee_error_t bungee_init(void);
void bungee_cleanup(void);

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

#ifdef __cplusplus
}
#endif

#endif // BUNGEE_C_H 