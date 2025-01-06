#include "bungee_c.h"
#include "bungee/Bungee.h"
#include <memory>

struct bungee_stretcher_t {
    std::unique_ptr<Bungee::Stretcher> stretcher;
};

extern "C" {

bungee_error_t bungee_init(void) {
    return BUNGEE_OK;
}

void bungee_cleanup(void) {
    // No cleanup needed currently
}

bungee_stretcher_t* bungee_create(bungee_sample_rates_t rates, int channels) {
    if (channels <= 0) {
        return nullptr;
    }

    try {
        auto stretcher = std::make_unique<Bungee::Stretcher>(rates.input_rate, rates.output_rate, channels);
        auto wrapper = new bungee_stretcher_t{std::move(stretcher)};
        return wrapper;
    } catch (...) {
        return nullptr;
    }
}

void bungee_destroy(bungee_stretcher_t* stretcher) {
    delete stretcher;
}

bungee_error_t bungee_preroll(bungee_stretcher_t* stretcher, const bungee_request_t* request) {
    if (!stretcher || !request) {
        return BUNGEE_NULL_POINTER;
    }

    try {
        Bungee::Request req;
        req.timeRatio = request->time_ratio;
        req.pitchScale = request->pitch_scale;
        req.windowSize = request->window_size;
        req.stepSize = request->step_size;
        
        stretcher->stretcher->preroll(req);
        return BUNGEE_OK;
    } catch (...) {
        return BUNGEE_INVALID_STATE;
    }
}

bungee_error_t bungee_specify_grain(bungee_stretcher_t* stretcher, const float* input_data, size_t frame_count) {
    if (!stretcher || !input_data) {
        return BUNGEE_NULL_POINTER;
    }

    try {
        stretcher->stretcher->specifyGrain(input_data, frame_count);
        return BUNGEE_OK;
    } catch (...) {
        return BUNGEE_INVALID_STATE;
    }
}

bungee_error_t bungee_analyse_grain(bungee_stretcher_t* stretcher) {
    if (!stretcher) {
        return BUNGEE_NULL_POINTER;
    }

    try {
        stretcher->stretcher->analyseGrain();
        return BUNGEE_OK;
    } catch (...) {
        return BUNGEE_INVALID_STATE;
    }
}

bungee_error_t bungee_synthesise_grain(bungee_stretcher_t* stretcher, float* output_data, size_t frame_count) {
    if (!stretcher || !output_data) {
        return BUNGEE_NULL_POINTER;
    }

    try {
        stretcher->stretcher->synthesiseGrain(output_data, frame_count);
        return BUNGEE_OK;
    } catch (...) {
        return BUNGEE_INVALID_STATE;
    }
}

bungee_error_t bungee_next(bungee_stretcher_t* stretcher) {
    if (!stretcher) {
        return BUNGEE_NULL_POINTER;
    }

    try {
        stretcher->stretcher->next();
        return BUNGEE_OK;
    } catch (...) {
        return BUNGEE_INVALID_STATE;
    }
}

bool bungee_is_flushed(const bungee_stretcher_t* stretcher) {
    if (!stretcher) {
        return true;
    }
    return stretcher->stretcher->isFlushed();
}

size_t bungee_max_input_frame_count(const bungee_stretcher_t* stretcher) {
    if (!stretcher) {
        return 0;
    }
    return stretcher->stretcher->maxInputFrameCount();
}

} // extern "C" 