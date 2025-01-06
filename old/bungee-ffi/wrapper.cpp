#include <bungee/Bungee.h>
#include "debug_log.h"
#include <cstdio>

#if defined(__APPLE__)
#define EXPORT __attribute__((visibility("default")))
#else
#define EXPORT
#endif

extern "C" {

// Get the function table for the basic implementation
static const Bungee_Stretcher_FunctionTable functionTable = Bungee::FunctionTable<Bungee::Basic>::get();

// Wrapper functions with logging
EXPORT void* Bungee_Stretcher_create_wrapper(Bungee_SampleRates rates, int channels, int log2SynthesisHop) {
    DBG_LOGF("Creating stretcher: sampleRate=%d, channels=%d, log2SynthesisHop=%d", 
             rates.input, channels, log2SynthesisHop);
    void* instance = functionTable.create(rates, channels, log2SynthesisHop);
    DBG_LOGF("Created stretcher instance: %p", instance);
    return instance;
}

EXPORT void Bungee_Stretcher_destroy_wrapper(void* instance) {
    DBG_LOGF("Destroying stretcher instance: %p", instance);
    functionTable.destroy(instance);
    DBG_LOG("Stretcher destroyed");
}

EXPORT void Bungee_Stretcher_analyseGrain_wrapper(void* instance, const float* data, intptr_t stride) {
    DBG_LOGF("Analyzing grain: instance=%p, data=%p, stride=%ld", instance, data, stride);
    functionTable.analyseGrain(instance, data, stride);
    DBG_LOG("Grain analysis complete");
}

EXPORT void Bungee_Stretcher_synthesiseGrain_wrapper(void* instance, Bungee_OutputChunk* chunk) {
    DBG_LOGF("Synthesizing grain: instance=%p, chunk=%p", instance, chunk);
    functionTable.synthesiseGrain(instance, chunk);
    DBG_LOGF("Grain synthesis complete: frameCount=%d", chunk->frameCount);
}

// Export the function table for FFI with logging
EXPORT Bungee_Stretcher_FunctionTable Bungee_Stretcher_getFunctionTable() {
    DBG_LOG("Getting function table");
    
    // Create a wrapped function table with logging
    Bungee_Stretcher_FunctionTable wrappedTable = {
        functionTable.version,
        Bungee_Stretcher_create_wrapper,
        Bungee_Stretcher_destroy_wrapper,
        functionTable.maxInputFrameCount,
        functionTable.preroll,
        functionTable.next,
        functionTable.specifyGrain,
        Bungee_Stretcher_analyseGrain_wrapper,
        Bungee_Stretcher_synthesiseGrain_wrapper,
        functionTable.isFlushed,
    };
    
    DBG_LOG("Returning wrapped function table");
    return wrappedTable;
}

// Initialize debug logging
EXPORT void Bungee_Stretcher_initDebugLog(const char* filename) {
    DebugLog::init(filename);
}

// Close debug logging
EXPORT void Bungee_Stretcher_closeDebugLog() {
    DebugLog::close();
}

} // extern "C" 