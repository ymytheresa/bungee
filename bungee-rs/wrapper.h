#pragma once
#include <memory>
#include <vector>
#include "rust/cxx.h"
#include "../bungee/bungee/Bungee.h"

namespace bungee_rs {
    // Forward declarations
    struct Request;
    struct InputChunk;
    struct OutputChunk;

    // We'll expose the Basic stretcher type
    class Stretcher : public Bungee::Stretcher<Bungee::Basic> {
    public:
        using Bungee::Stretcher<Bungee::Basic>::Stretcher;  // Inherit constructors

        void preroll(Request& request) const;
        InputChunk specify_grain(const Request& request) const;
        void analyse_grain(rust::Slice<const float> data, int32_t channel_stride) const;
        OutputChunk synthesise_grain() const;
        void next(Request& request) const;
    };

    // Factory function
    std::unique_ptr<Stretcher> new_stretcher(int32_t sample_rate, int32_t channels);
} 