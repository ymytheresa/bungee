#ifndef BUNGEE_C_H
#define BUNGEE_C_H

#include <stddef.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

// Debug output control
#ifdef BUNGEE_DEBUG
#define BUNGEE_LOG(fmt, ...) fprintf(stderr, "[BUNGEE] " fmt "\n", __VA_ARGS__)
#define BUNGEE_LOG_SIMPLE(msg) fprintf(stderr, "[BUNGEE] " msg "\n")
#else
#define BUNGEE_LOG(fmt, ...) ((void)0)
#define BUNGEE_LOG_SIMPLE(msg) ((void)0)
#endif

// Error codes
typedef enum bungee_error {
    BUNGEE_OK = 0,
    BUNGEE_NULL_POINTER,
    BUNGEE_INVALID_PARAM,
    BUNGEE_MEMORY,
    BUNGEE_INVALID_STATE,
    BUNGEE_BUFFER_TOO_SMALL
} bungee_error_t;

// Audio configuration
typedef struct {
    int input_rate;
    int output_rate;
} bungee_sample_rates_t;

// Processing request
typedef struct {
    double position;  // Frame-offset within input audio
    double speed;     // Output audio speed (1.0 = normal)
    double pitch;     // Pitch adjustment (1.0 = normal)
    bool reset;       // Reset flag
} bungee_request_t;

// Input/Output chunk descriptors
typedef struct {
    int32_t begin;
    int32_t end;
} bungee_input_chunk_t;

typedef struct {
    float* data;
    int32_t frame_count;
    size_t channel_stride;
} bungee_output_chunk_t;

// Opaque handle to the stretcher
typedef struct bungee_stretcher bungee_stretcher_t;

// Core functions
bungee_error_t bungee_init(void);
void bungee_cleanup(void);

// Creation/Destruction
bungee_stretcher_t* bungee_create(bungee_sample_rates_t rates, int channels);
void bungee_destroy(bungee_stretcher_t* stretcher);

// Processing functions
bungee_error_t bungee_preroll(bungee_stretcher_t* stretcher, const bungee_request_t* request);
bungee_error_t bungee_specify_grain(bungee_stretcher_t* stretcher, const float* input_data, size_t frame_count, bungee_input_chunk_t* chunk);
bungee_error_t bungee_analyse_grain(bungee_stretcher_t* stretcher, const float* input_data, size_t channel_stride);
bungee_error_t bungee_synthesise_grain(bungee_stretcher_t* stretcher, bungee_output_chunk_t* chunk);
bungee_error_t bungee_next(bungee_stretcher_t* stretcher, bungee_request_t* request);

// Query functions
bool bungee_is_flushed(const bungee_stretcher_t* stretcher);
size_t bungee_max_input_frame_count(const bungee_stretcher_t* stretcher);

#endif // BUNGEE_C_H 