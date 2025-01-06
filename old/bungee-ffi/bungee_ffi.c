#include "bungee_ffi.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// Memory tracking
static size_t allocated_memory = 0;
static size_t allocation_count = 0;

// Debug logging
static FILE* debug_log = NULL;

// Memory management functions
static void* track_alloc(size_t size) {
    void* ptr = malloc(size);
    if (ptr) {
        allocated_memory += size;
        allocation_count++;
    }
    return ptr;
}

static void track_free(void* ptr, size_t size) {
    if (ptr) {
        allocated_memory -= size;
        allocation_count--;
        free(ptr);
    }
}

// Debug logging
static void log_debug(const char* msg) {
    if (debug_log) {
        fprintf(debug_log, "%s\n", msg);
        fflush(debug_log);
    }
}

// FFI functions
bungee_error_t bungee_init(void) {
    log_debug("Initializing bungee FFI");
    return BUNGEE_OK;
}

void bungee_cleanup(void) {
    log_debug("Cleaning up bungee FFI");
    if (allocated_memory != 0 || allocation_count != 0) {
        fprintf(stderr, "Memory leak detected: %zu bytes in %zu blocks\n",
                allocated_memory, allocation_count);
    }
}

bungee_stretcher_t* bungee_create(bungee_sample_rates_t rates, int channels) {
    if (channels <= 0) {
        log_debug("Invalid channel count");
        return NULL;
    }

    bungee_stretcher_t* stretcher = track_alloc(sizeof(bungee_stretcher_t));
    if (!stretcher) {
        log_debug("Failed to allocate stretcher");
        return NULL;
    }

    stretcher->rates = rates;
    stretcher->channels = channels;
    stretcher->state = BUNGEE_STATE_CREATED;

    log_debug("Created stretcher instance");
    return stretcher;
}

void bungee_destroy(bungee_stretcher_t* stretcher) {
    if (!stretcher) {
        log_debug("Attempt to destroy NULL stretcher");
        return;
    }

    log_debug("Destroying stretcher instance");
    track_free(stretcher, sizeof(bungee_stretcher_t));
}

bungee_error_t bungee_preroll(bungee_stretcher_t* stretcher, const bungee_request_t* request) {
    if (!stretcher || !request) {
        log_debug("NULL pointer in preroll");
        return BUNGEE_ERROR_NULL_POINTER;
    }

    if (stretcher->state != BUNGEE_STATE_CREATED) {
        log_debug("Invalid stretcher state for preroll");
        return BUNGEE_ERROR_INVALID_STATE;
    }

    stretcher->state = BUNGEE_STATE_PREROLLED;
    log_debug("Preroll completed");
    return BUNGEE_OK;
}

bungee_error_t bungee_specify_grain(bungee_stretcher_t* stretcher, const float* input_data, size_t frame_count) {
    if (!stretcher || !input_data) {
        log_debug("NULL pointer in specify_grain");
        return BUNGEE_ERROR_NULL_POINTER;
    }

    if (frame_count == 0) {
        log_debug("Zero frame count in specify_grain");
        return BUNGEE_ERROR_INVALID_PARAM;
    }

    if (stretcher->state != BUNGEE_STATE_PREROLLED) {
        log_debug("Invalid stretcher state for specify_grain");
        return BUNGEE_ERROR_INVALID_STATE;
    }

    stretcher->state = BUNGEE_STATE_GRAIN_SPECIFIED;
    log_debug("Grain specified");
    return BUNGEE_OK;
}

bungee_error_t bungee_analyse_grain(bungee_stretcher_t* stretcher) {
    if (!stretcher) {
        log_debug("NULL pointer in analyse_grain");
        return BUNGEE_ERROR_NULL_POINTER;
    }

    if (stretcher->state != BUNGEE_STATE_GRAIN_SPECIFIED) {
        log_debug("Invalid stretcher state for analyse_grain");
        return BUNGEE_ERROR_INVALID_STATE;
    }

    stretcher->state = BUNGEE_STATE_GRAIN_ANALYSED;
    log_debug("Grain analysed");
    return BUNGEE_OK;
}

bungee_error_t bungee_synthesise_grain(bungee_stretcher_t* stretcher, float* output_data, size_t frame_count) {
    if (!stretcher || !output_data) {
        log_debug("NULL pointer in synthesise_grain");
        return BUNGEE_ERROR_NULL_POINTER;
    }

    if (frame_count == 0) {
        log_debug("Zero frame count in synthesise_grain");
        return BUNGEE_ERROR_INVALID_PARAM;
    }

    if (stretcher->state != BUNGEE_STATE_GRAIN_ANALYSED) {
        log_debug("Invalid stretcher state for synthesise_grain");
        return BUNGEE_ERROR_INVALID_STATE;
    }

    stretcher->state = BUNGEE_STATE_GRAIN_SYNTHESISED;
    log_debug("Grain synthesised");
    return BUNGEE_OK;
}

bungee_error_t bungee_next(bungee_stretcher_t* stretcher) {
    if (!stretcher) {
        log_debug("NULL pointer in next");
        return BUNGEE_ERROR_NULL_POINTER;
    }

    if (stretcher->state != BUNGEE_STATE_GRAIN_SYNTHESISED) {
        log_debug("Invalid stretcher state for next");
        return BUNGEE_ERROR_INVALID_STATE;
    }

    stretcher->state = BUNGEE_STATE_PREROLLED;
    log_debug("Advanced to next grain");
    return BUNGEE_OK;
}

bool bungee_is_flushed(const bungee_stretcher_t* stretcher) {
    if (!stretcher) {
        log_debug("NULL pointer in is_flushed");
        return false;
    }

    return stretcher->state == BUNGEE_STATE_CREATED;
}

size_t bungee_max_input_frame_count(const bungee_stretcher_t* stretcher) {
    if (!stretcher) {
        log_debug("NULL pointer in max_input_frame_count");
        return 0;
    }

    return 4096; // Example fixed size, adjust based on actual requirements
}

void bungee_print_memory_stats(void) {
    printf("Memory stats:\n");
    printf("  Allocated: %zu bytes\n", allocated_memory);
    printf("  Blocks: %zu\n", allocation_count);
}

void bungee_set_debug_log(FILE* log_file) {
    debug_log = log_file;
    log_debug("Debug logging initialized");
} 