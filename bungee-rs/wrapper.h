#pragma once
#include <memory>
#include <vector>
#include "rust/cxx.h"
#include "../bungee/bungee/Bungee.h"

namespace bungee_rs {
    struct Request;
    struct InputChunk;
    struct OutputChunk;

    class Stretcher {
    private:
        // Use unique_ptr to manage the underlying Bungee stretcher
        std::unique_ptr<Bungee::Stretcher<Bungee::Basic>> impl;

    public:
        Stretcher(std::pair<int32_t, int32_t> rates, int32_t channels)
            : impl(std::make_unique<Bungee::Stretcher<Bungee::Basic>>(
                Bungee::SampleRates{rates.first, rates.second}, 
                channels)) {}

        // All methods modify state, so they should not be const
        void preroll(Request& request);
        InputChunk specify_grain(const Request& request);
        void analyse_grain(rust::Slice<const float> data, int32_t channel_stride);
        OutputChunk synthesise_grain();
        void next(Request& request);
    };

    std::unique_ptr<Stretcher> new_stretcher(int32_t sample_rate, int32_t channels);
} 