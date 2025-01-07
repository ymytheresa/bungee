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

### Audio Processing
1. **Zero Output Samples**
   - Issue: Audio processing produces 0 output samples
   - Root Cause: Potential issues:
     * Incorrect grain size calculation
     * Buffer management problems
     * Error in C implementation not being caught
   - Fix Needed: Debug audio processing chain
   - Lesson: Add debug logging and validation for audio processing

## Future Prevention
1. **Build System**
   - [ ] Add library path verification step
   - [ ] Document library locations and search paths
   - [ ] Add build script debug output

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

## Checklist for New FFI Work
1. [ ] Verify library paths and build configuration
2. [ ] Check bindgen-generated types
3. [ ] Test error code conversions
4. [ ] Verify memory management
5. [ ] Add comprehensive tests
6. [ ] Document all assumptions and requirements
7. [ ] Add debug logging for critical operations
8. [ ] Validate data processing results 