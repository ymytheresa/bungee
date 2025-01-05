#include "wrapper.h"
#include "bungee.rs.h"
#include <memory>
#include <iostream>

namespace bungee_rs {

std::unique_ptr<Stretcher> new_stretcher(int32_t sample_rate, int32_t channels) {
    return std::make_unique<Stretcher>(
        std::pair<int32_t, int32_t>{sample_rate, sample_rate}, 
        channels
    );
}

void Stretcher::preroll(Request& request) {
    impl->preroll(request);
}

InputChunk Stretcher::specify_grain(const Request& request) {
    return impl->specify_grain(request);
}

void Stretcher::analyse_grain(rust::Slice<const float> data, int32_t channel_stride) {
    impl->analyse_grain(data, channel_stride);
}

OutputChunk Stretcher::synthesise_grain() {
    return impl->synthesise_grain();
}

void Stretcher::next(Request& request) {
    impl->next(request);
}

} // namespace bungee_rs 