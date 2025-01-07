# Bungee FFI Project Plan

## Primary Goal
The main goal of this project is to enable Rust to call the Bungee audio processing library effectively and safely. This means:
- Creating a pure C FFI layer that Rust can call
- Ensuring all Bungee functionality is accessible from Rust
- Providing a safe and idiomatic Rust API
- Maintaining performance while crossing FFI boundaries
- Proper error handling between Rust and C

## Understanding Bungee
Bungee is a C++ audio time-stretching and pitch-shifting library that:
- Processes audio in grains (chunks)
- Supports real-time processing
- Uses a template-based implementation for flexibility
- Provides both basic and pro implementations
- Already has a C-compatible interface layer through `Bungee_Stretcher_FunctionTable`

## Core Goal
Create a minimal, pure C FFI layer that serves as a clean parameter-passing interface between Rust and the Bungee C++ library. The FFI should:
- Use the existing C-compatible interface in Bungee.h
- Maintain the grain-based processing model
- Preserve the real-time processing capabilities
- Support both time-stretching and pitch-shifting operations

## Key Principles
1. **Simplicity First**
   - The wrapper should ONLY handle parameter passing
   - No business logic in the wrapper layer
   - Minimal type conversions
   - No unnecessary abstractions
   - Use existing C interface from Bungee.h

2. **Pure C FFI**
   - Use C ABI for stability and compatibility
   - Avoid C++ features in the interface
   - Keep the interface as C-like as possible
   - No exceptions across FFI boundary
   - Leverage existing `extern "C"` functions

3. **Minimal Dependencies**
   - No external libraries in the FFI layer
   - Use only necessary standard library features
   - Keep build system simple
   - Rely on Bungee's existing build system

## Core Files to Focus On
1. **Bungee Library Files**
   - `bungee/Bungee.h` - Main interface definition
     * Contains `Bungee_Stretcher_FunctionTable` for C interface
     * Defines core types like `Bungee_Request`
     * Already provides `extern "C"` functions
   - `bungee/Basic.h` - Basic implementation details
   - Focus on the existing C-compatible interfaces:
     * `Bungee_Stretcher_FunctionTable` - Core function table
     * `Bungee_Request` - Audio processing parameters
     * `Bungee_InputChunk` - Input audio chunk definition
     * `Bungee_OutputChunk` - Output audio chunk definition

2. **FFI Implementation Files**
   - Single header for C interface declarations
   - Single implementation file for the FFI layer
   - Minimal Rust binding file
   - Focus on grain-based audio processing

## Audio Processing Model
1. **Grain-Based Processing**
   - Audio is processed in chunks (grains)
   - Each grain has:
     * Position in the input audio
     * Speed (time-stretching factor)
     * Pitch adjustment
   - Grains can overlap for smooth transitions

2. **Parameter Handling**
   - Time-stretching: Speed parameter (1.0 = normal speed)
   - Pitch-shifting: Pitch parameter (1.0 = original pitch)
   - Position tracking for real-time processing
   - Reset flag for state management

## What We Don't Want
1. **No Complex Abstractions**
   - No additional C++ wrapper classes
   - No template metaprogramming
   - No inheritance in the FFI layer
   - No virtual functions

2. **No Feature Creep**
   - No additional functionality beyond Bungee's core features
   - No "nice to have" extensions
   - No performance optimizations in the FFI layer
   - No additional type safety beyond what's necessary

3. **No Build Complexity**
   - No complex CMake configurations
   - No unnecessary compiler flags
   - No platform-specific code unless absolutely required
   - No optional features

## Implementation Approach
1. **FFI Layer**
   - Create minimal C header defining the interface
   - Implement straightforward C functions that:
     * Take C-compatible types
     * Pass parameters to Bungee
     * Return C-compatible types
   - Handle errors through return values, not exceptions

2. **Memory Management**
   - Clear ownership rules
   - No smart pointers in the interface
   - Explicit resource cleanup functions
   - No hidden allocations

3. **Error Handling**
   - Use simple error codes
   - No exception propagation
   - Clear error states
   - Explicit error checking

## Testing Strategy
1. **Unit Tests**
   - Test parameter passing
   - Test error conditions
   - Test memory management
   - No complex test frameworks

2. **Integration Tests**
   - Basic audio processing tests
   - Memory leak checks
   - Error propagation verification

## Documentation
1. **Interface Documentation**
   - Clear function descriptions
   - Parameter documentation
   - Error conditions
   - Usage examples

2. **Build Instructions**
   - Simple step-by-step guide
   - Clear dependencies list
   - Basic troubleshooting

## Success Criteria
1. Successfully pass audio processing parameters between Rust and C++
2. No memory leaks
3. Proper error propagation
4. Clean build process
5. Clear documentation
6. Basic test coverage

## Non-Goals
1. Performance optimization
2. Additional features
3. Platform-specific optimizations
4. Complex build configurations
5. Advanced error handling
6. Automatic memory management 

## Implementation Details
1. **Core Functions to Wrap**
   - Initialization and cleanup
   - Stretcher creation/destruction
   - Grain specification and analysis
   - Audio synthesis
   - State queries (flushed state, frame counts)

2. **Parameter Conversion**
   - Simple numeric parameter passing
   - No complex type conversions
   - Direct mapping to Bungee's C interface
   - Clear error code propagation

3. **Memory Layout**
   - Respect Bungee's memory layout
   - No additional memory management
   - Direct buffer passing
   - Clear ownership boundaries 

WE ARE DOING PURE C FFI!! DONT USE ANY C++ FEATURES!!