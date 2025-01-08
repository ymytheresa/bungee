# Missed Issues and Lessons Learned

## Build System Issues

### Library Path Issues
1. **Library Search Path**
   - Issue: Linker couldn't find `libbungee_c.a`
   - Root Cause: Incorrect library search path in build.rs
   - Fix Needed: Update library path to point to correct location
   - Lesson: Always verify library paths in both CMake and Cargo build scripts

### Type Mismatches
1. **C to Rust Enum Conversion**
   - Issue: C enum was translated to `u32` by bindgen but we tried to use `i32`
   - Root Cause: Not checking bindgen's generated types
   - Fix: Updated error conversion to use correct type (`u32`)
   - Lesson: Always check bindgen's generated code for correct type mappings

### FFI Interface
1. **Error Code Handling**
   - Issue: Initially used enum variants directly, but they were actually numeric values
   - Root Cause: Not checking how C enums are represented in Rust
   - Fix: Use numeric values for error code comparison
   - Lesson: Remember C enums are just integers with names

2. **Struct Field Initialization**
   - Issue: Segmentation fault due to accessing uninitialized struct field
   - Root Cause: Struct definition had `output_buffer` field but initialization code was removed
   - Fix: Remove unused field from struct definition
   - Lesson: Keep struct definitions and initialization code in sync
   - Prevention:
     * Review all struct fields during refactoring
     * Document field dependencies
     * Add initialization checklist for each struct
     * Consider using designated initializers in C99/C11

3. **Audio Data Layout**
   - Issue: Confusion about channel stride in interleaved audio
   - Root Cause: Not clearly documenting audio data layout expectations
   - Investigation: Tried changing channel_stride from 1 to 2 for stereo
   - Lesson: Document and validate audio data layout assumptions
   - Prevention:
     * Document interleaving format (e.g., [L R L R] vs [L L R R])
     * Add channel stride validation
     * Create test cases for different audio layouts
     * Add debug logging for audio buffer access patterns

### Audio Processing
1. **Zero Output Samples**
   - Issue: Audio processing produces 0 output samples
   - Root Cause: Potential issues:
     * Incorrect grain size calculation
     * Buffer management problems
     * Error in C implementation not being caught
     * Channel stride/interleaving issues
   - Fix Needed: Debug audio processing chain
   - Lesson: Add debug logging and validation for audio processing

2. **Debug Logging**
   - Issue: Debug logging not appearing despite BUNGEE_DEBUG being defined
   - Root Cause: Multiple potential issues:
     * CMake debug flags not properly propagating
     * Macro definition not reaching C code
     * Build configuration issues
   - Fix Needed: Verify debug flag propagation
   - Lesson: Test debug logging early in development

3. **Test Signal Design**
   - Issue: Test signal properties not fully leveraged for debugging
   - Root Cause: Not documenting test signal characteristics
   - Fix Needed: Add validation based on known signal properties:
     * Known frequency (440 Hz)
     * Known amplitude range (-1.0 to 1.0)
     * Identical stereo channels
     * Continuous sine wave
   - Lesson: Document and validate test signal properties

## Future Prevention
1. **Build System**
   - [ ] Add library path verification step
   - [ ] Document library locations and search paths
   - [ ] Add build script debug output
   - [ ] Verify debug flag propagation
   - [ ] Add debug build validation

2. **Type Safety**
   - [ ] Review bindgen output before implementing FFI wrappers
   - [ ] Document type mappings between C and Rust
   - [ ] Add type verification tests

3. **Error Handling**
   - [ ] Document error code values and meanings
   - [ ] Add error code conversion tests
   - [ ] Verify error propagation across FFI boundary

4. **Audio Processing**
   - [ ] Add debug logging for audio buffer sizes
   - [ ] Validate input/output sample counts
   - [ ] Add audio processing tests with known outputs
   - [ ] Implement audio validation tools
   - [ ] Add frequency domain analysis
   - [ ] Validate stereo channel consistency
   - [ ] Check signal amplitude preservation
   - [ ] Document audio data layout requirements
   - [ ] Add buffer access pattern validation
   - [ ] Create test cases for different channel layouts

5. **Struct Management**
   - [ ] Create initialization checklist for each struct
   - [ ] Document field dependencies and lifecycle
   - [ ] Add memory layout validation tests
   - [ ] Consider using static analysis tools
   - [ ] Add struct field usage tracking
   - [ ] Implement struct validation functions

## Checklist for New FFI Work
1. [ ] Verify library paths and build configuration
2. [ ] Check bindgen-generated types
3. [ ] Test error code conversions
4. [ ] Verify memory management
5. [ ] Add comprehensive tests
6. [ ] Document all assumptions and requirements
7. [ ] Add debug logging for critical operations
8. [ ] Validate data processing results
9. [ ] Test debug logging functionality
10. [ ] Document test signal properties
11. [ ] Review struct initialization code
12. [ ] Validate struct field usage
13. [ ] Document and validate data layouts
14. [ ] Test with different data organizations 