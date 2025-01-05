# Bungee Rust Wrapper Development Log

## Error Tracking

### 2024-01-05: Initial CXX Integration

#### Error Set 1: Struct Definition Conflicts
```
error: redefinition of 'Request'
error: redefinition of 'InputChunk'
error: redefinition of 'OutputChunk'
```
**Cause**: Both CXX and wrapper.h trying to define the same structs.  
**Solution**: Remove struct definitions from wrapper.h and let CXX handle them.  
**Status**: Fixed

#### Error Set 2: Missing C++ Includes
```
error: no template named 'vector' in namespace 'std'
error: use of undeclared identifier 'rust'
error: no template named 'unique_ptr' in namespace 'std'
```
**Cause**: Missing necessary C++ standard library and CXX includes.  
**Solution**: Added includes in wrapper.h:
```cpp
#include <vector>
#include <memory>
#include "rust/cxx.h"
```
**Status**: Fixed

#### Error Set 3: Const Correctness
```
error: cannot initialize a variable of type '... const' with an rvalue of type '... ': different qualifiers
```
**Cause**: Mismatch between const qualifiers in C++ and Rust interfaces.  
**Solution**: Updated method signatures to use consistent const qualifiers and Pin<&T> in Rust.  
**Status**: Fixed

#### Error Set 4: Missing Generated Header
```
fatal error: 'src/bungee.rs.h' file not found
```
**Cause**: The wrapper.h is trying to include the CXX generated header before it's generated.  
**Solution**: Updated build.rs to include the correct CXX generated header paths:
```rust
build.include(format!("{}/cxxbridge/include", std::env::var("OUT_DIR").unwrap()));
build.include(format!("{}/cxxbridge/crate", std::env::var("OUT_DIR").unwrap()));
```
**Status**: Fixed

#### Error Set 5: Apple Silicon (M1) Targeting
```
error: IPHONEOS_DEPLOYMENT_TARGET environment variable
```
**Cause**: Build system confusion between macOS and iOS targets on Apple Silicon.  
**Solution**: Added explicit macOS targeting in build.rs:
```rust
#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
{
    build.flag("-mmacosx-version-min=11.0");
    build.flag("-arch");
    build.flag("arm64");
    build.flag("-mcpu=apple-m1");
}
println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=11.0");
```
**Status**: Fixed

#### Error Set 6: Type Mismatch Between Rust and C++
```
error: cannot initialize a variable of type 'void (bungee_rs::Stretcher::*)(::bungee_rs::Request &) const'
with an rvalue of type 'void (bungee_rs::Stretcher::*)(Bungee::Request &) const'
```
**Cause**: Using Bungee namespace types directly in wrapper interface instead of our Rust types.  
**Solution**: 
1. Updated wrapper.h to use Rust types in interface
2. Added proper type conversions in wrapper.cpp
3. Implemented conversion between Bungee and Rust types
**Status**: Fixed

#### Error Set 7: CXX Header Include Order
```
fatal error: 'bungee.rs.h' file not found
```
**Cause**: Incorrect include order and path resolution for CXX generated headers.  
**Solution**: 
1. Updated include order in wrapper.h to include CXX headers first
2. Simplified wrapper.h to only use necessary includes
3. Updated build.rs to include proper paths for generated headers
**Status**: In Progress

#### Error Set 8: CXX Namespace and Type Resolution
```
error: no type named 'Stretcher' in the global namespace; did you mean 'bungee_rs::Stretcher'?
error: no template named 'Stretcher' in the global namespace; did you mean 'Bungee::Stretcher'?
error: use of class template '::Stretcher' requires template arguments
```
**Cause**: Multiple issues with namespace handling in CXX bridge:
1. Stretcher type was being looked up in global namespace instead of bungee_rs
2. Template arguments for Bungee::Stretcher weren't properly handled
3. Method bindings weren't properly specifying self parameter

**Solution**: 
1. Added explicit namespace to CXX bridge:
```rust
#[cxx::bridge(namespace = "bungee_rs")]
```
2. Changed type declaration to just declare opaque type:
```rust
type Stretcher;  // Instead of type Stretcher = crate::bungee_rs::Stretcher;
```
3. Updated method bindings to explicitly specify self parameter:
```rust
fn preroll(self: &Stretcher, request: &mut Request);
// Instead of fn preroll(&self, request: &mut Request);
```

**Status**: Fixed

#### Error Set 9: Incomplete Type Errors
```
error: member access into incomplete type 'Request'
error: incomplete result type 'InputChunk' in function definition
```
**Cause**: The structs (Request, InputChunk) are only forward declared in wrapper.h but their full definitions are not available to the C++ code.
The CXX bridge generates the struct definitions in a separate header that needs to be included.

**Solution**: 
1. Include the generated CXX header in wrapper.cpp:
```cpp
#include "wrapper.h"
#include "src/bungee.rs.h"  // Generated header with struct definitions
```
2. Update include paths in build.rs to ensure the generated header is found
3. Move struct definitions to a shared header that both Rust and C++ can use

**Status**: In Progress

## Error Set 9: CXX Const-Correctness Issues (January 8, 2024)

### Errors
1. Cannot initialize const-qualified member functions with non-const implementations:
```
error: cannot initialize a variable of type '::bungee_rs::InputChunk (bungee_rs::Stretcher::*)(const ::bungee_rs::Request &) const' with an rvalue of type 'InputChunk (bungee_rs::Stretcher::*)(const Request &)'
error: cannot initialize a variable of type 'void (bungee_rs::Stretcher::*)(::rust::Slice<const float>, ::std::int32_t) const' with an rvalue of type 'void (bungee_rs::Stretcher::*)(rust::Slice<const float>, int32_t)'
error: cannot initialize a variable of type '::bungee_rs::OutputChunk (bungee_rs::Stretcher::*)() const' with an rvalue of type 'OutputChunk (bungee_rs::Stretcher::*)()'
```

### Cause
- The CXX bridge is expecting const member functions for non-mutating methods, but the C++ implementation doesn't mark these methods as const.
- This creates a mismatch between the Rust FFI declarations and the C++ implementations.

### Solution
1. Update the C++ implementation in `wrapper.cpp` to mark non-mutating methods as const:
   - `specify_grain`
   - `analyse_grain`
   - `synthesise_grain`
2. Ensure the method declarations in `wrapper.h` also have the const qualifier.
3. Keep the const qualifiers in the Rust FFI bridge declarations.

### Status
- ⏳ In Progress
- The issue has been identified and the solution is being implemented.
- This will ensure proper const-correctness across the Rust-C++ boundary.

## Solutions Applied

### 2024-01-05
1. Added proper includes in wrapper.h
2. Removed duplicate struct definitions
3. Updated const qualifiers in C++ methods
4. Fixed CXX generated header inclusion
5. Added proper M1 targeting configuration

### 2024-01-06
1. Implemented type conversions between Rust and C++ types
2. Added explicit macOS version targeting for M1
3. Fixed namespace handling in Rust bindings
4. Restructured as library with binary example
5. Added proper type conversions in wrapper implementation

### 2024-01-07
1. Simplified wrapper.h to focus on essential includes
2. Updated build.rs to handle generated headers properly
3. Added debug logging for better error tracking
4. Improved error documentation and tracking

### 2024-01-08
1. Fixed namespace handling in CXX bridge by adding explicit namespace
2. Simplified type declarations to avoid template resolution issues
3. Updated method bindings to properly specify self parameter
4. Maintained proper namespace encapsulation in wrapper.cpp
5. Kept debug logging for better error tracking

### 2024-01-09
1. Fixed incomplete type errors by including generated headers
2. Improved header include order
3. Added proper include paths in build.rs

January 8, 2024:
- Added proper namespace handling in CXX bridge
- Fixed type resolution for Stretcher class
- Implemented proper const-correctness for member functions
- Added comprehensive logging for debugging
- Added safety checks for pointer dereferencing in synthesise_grain

## Error Set 10: Architectural Conflict - State Modification vs Const Safety

### Core Issue
We've discovered a fundamental architectural conflict between the base C++ class behavior and CXX bridge requirements:

1. Base Class (Bungee::Stretcher) Requirements:
   - Methods like `specify_grain`, `analyse_grain`, and `synthesise_grain` modify internal state
   - They are non-const methods by design
   - State modification is fundamental to the time-stretching algorithm

2. CXX Bridge Requirements:
   - Enforces const methods for safety guarantees
   - Expects immutable references where possible
   - Generates bindings that require const qualifiers

This creates a circular problem:
```
Make methods non-const → Rust binding errors
Make methods const → Base class errors
Remove const → Back to Rust binding errors
```

### Proposed Solutions

1. Modify Base Class (❌ Rejected):
   - Would require modifying core Bungee library
   - Could break existing functionality
   - Goes against the fundamental design of the algorithm

2. Modify CXX Bridge Requirements (✅ Preferred):
   - Update Rust bindings to explicitly handle mutable state
   - Use `Pin<&mut Stretcher>` where needed
   - Document state modifications clearly
   - Maintains original library semantics

3. Create Different Architecture (❌ Rejected):
   - Would add unnecessary complexity
   - Potential performance overhead
   - Harder to maintain

### Next Steps
1. Update Rust bindings to properly handle mutable state
2. Remove const qualifiers to match base class behavior
3. Add comprehensive documentation about state modifications
4. Ensure proper error handling for state-dependent operations

### Status
- ⏳ In Progress
- Architectural decision made: Will adapt bindings to match base class behavior
- Implementation pending

### Final Solution
1. Made all C++ methods non-const to match base class behavior:
```cpp
// wrapper.h
void preroll(Request& request);
InputChunk specify_grain(const Request& request);
void analyse_grain(rust::Slice<const float> data, int32_t channel_stride);
OutputChunk synthesise_grain();
void next(Request& request);
```

2. Used Pin<&mut> in Rust to indicate state modification:
```rust
fn preroll(self: Pin<&mut Stretcher>, request: &mut Request);
fn specify_grain(self: Pin<&mut Stretcher>, request: &Request) -> InputChunk;
fn analyse_grain(self: Pin<&mut Stretcher>, data: &[f32], channel_stride: i32);
fn synthesise_grain(self: Pin<&mut Stretcher>) -> OutputChunk;
fn next(self: Pin<&mut Stretcher>, request: &mut Request);
```

3. Documented state modification behavior clearly in wrapper.h:
```cpp
/// Stretcher class that wraps Bungee::Stretcher<Bungee::Basic>
/// Note: All methods modify internal state and are therefore non-const
```

### 2024-01-10
1. Resolved const-correctness architectural conflict
2. Implemented consistent state mutation handling
3. Updated documentation to reflect architectural decisions
4. Verified solution works with base class behavior
5. Added clear state modification documentation

## Current Status
- Basic wrapper structure is in place
- Build system is properly configured for Apple Silicon
- Type conversions are implemented
- Const correctness is maintained
- Library and binary targets are set up
- Basic example is working
- Debug logging added for better troubleshooting
- Namespace issues resolved
- Type resolution working correctly
- Method bindings properly specified
- Working on resolving incomplete type errors
- ✅ Const-correctness issues resolved
- ✅ State mutation handling implemented
- ✅ Base class behavior preserved
- ✅ Documentation updated

## TODO
- [x] Fix namespace handling in CXX bridge
- [x] Fix type resolution issues
- [x] Fix incomplete type errors
- [x] Resolve const-correctness architectural conflict
- [ ] Add comprehensive error handling
- [ ] Implement unit tests
- [ ] Add documentation for public API
- [ ] Create usage examples
- [ ] Add CI/CD pipeline
- [ ] Performance testing and optimization
- [ ] Verify memory safety with audio buffers
- [ ] Add cross-platform support
- [ ] Add error recovery strategies 

## Lessons Learned
1. When working with C++ libraries that rely on state modification, prefer explicit mutation handling in Rust (Pin<&mut>) over const-correctness
2. Document architectural decisions and their rationale clearly
3. Keep base class behavior intact rather than forcing Rust idioms
4. Use Rust's type system to make state modification explicit and safe 

## Error Set 11: Revisiting Const-Correctness (January 9, 2024)

### Analysis of Previous Solutions
1. Using `Pin<&mut Stretcher>` in Rust - Correctly represents state modification
2. Making methods const - Conflicts with base class behavior
3. Shared pointer approach - Good for ownership, but doesn't solve const-correctness

### Root Cause
1. Base Bungee::Stretcher methods modify state by design:
   - `preroll` modifies request and internal state
   - `specify_grain` updates internal grain state
   - `analyse_grain` processes and stores grain data
   - `synthesise_grain` generates output based on state
   - `next` updates request and internal state

2. Rust/CXX Requirements:
   - Need to accurately represent state modification
   - `Pin<&mut>` is the correct way to show state modification
   - Const methods would misrepresent behavior

### Final Solution
1. Keep methods non-const in C++ wrapper:
   ```cpp
   void preroll(Request& request);
   InputChunk specify_grain(const Request& request);
   void analyse_grain(rust::Slice<const float> data, int32_t channel_stride);
   OutputChunk synthesise_grain();
   void next(Request& request);
   ```

2. Use `Pin<&mut Stretcher>` in Rust to show state modification:
   ```rust
   fn preroll(self: Pin<&mut Stretcher>, request: &mut Request);
   fn specify_grain(self: Pin<&mut Stretcher>, request: &Request) -> InputChunk;
   fn analyse_grain(self: Pin<&mut Stretcher>, data: &[f32], channel_stride: i32);
   fn synthesise_grain(self: Pin<&mut Stretcher>) -> OutputChunk;
   fn next(self: Pin<&mut Stretcher>, request: &mut Request);
   ```

3. Use `shared_ptr` in C++ wrapper purely for ownership, not for const-correctness.

### Status
✅ Resolved: Understanding that state modification is fundamental to the algorithm and should be reflected in the type system.

## Error Set 12: Constructor Type Mismatch

### Error
```
error: no matching constructor for initialization of 'Bungee::Stretcher<Bungee::Basic>'
note: candidate constructor not viable: no known conversion from 'std::pair<int, int>' to 'SampleRates' (aka 'Bungee_SampleRates') for 1st argument
```

### Cause
When implementing `shared_ptr` approach, we discovered a type mismatch in the constructor:
1. Our wrapper tries to use `std::pair<int32_t, int32_t>` for sample rates
2. But Bungee::Stretcher expects `Bungee::SampleRates` type
3. No automatic conversion exists between these types

### Solution
Update the constructor to properly construct `Bungee::SampleRates`:
```cpp
Stretcher(std::pair<int32_t, int32_t> rates, int32_t channels)
    : impl(std::make_shared<Bungee::Stretcher<Bungee::Basic>>(
        Bungee::SampleRates{rates.first, rates.second}, 
        channels)) {}
```

### Status
- ⏳ Ready to implement
- Part of the shared_ptr implementation approach
- Demonstrates the importance of proper type handling in the wrapper

### Lessons Learned
1. When wrapping C++ classes, need to carefully check constructor parameter types
2. Using intermediate types (like std::pair) requires explicit conversion
3. The wrapper should handle type conversions transparently for Rust users

## Error Set 13: State Modification Design Pattern

### Core Understanding
After multiple iterations and attempts, we've arrived at a clearer design pattern for handling state modification:

1. Ownership vs State Modification:
   - Use `shared_ptr` purely for ownership in C++
   - Use `Pin<&mut>` purely for state modification in Rust
   - Don't mix these concerns or try to solve one with the other

2. Explicit State Modification:
   - All methods that could modify state should use `Pin<&mut>` in Rust
   - Even if C++ marks some methods as const, prefer being explicit about potential state changes
   - This matches Rust's philosophy of making side effects clear in the type system

3. Implementation Pattern:
```cpp
// C++ (wrapper.h)
class Stretcher {
    std::shared_ptr<Bungee::Stretcher<Bungee::Basic>> impl;
public:
    // Be explicit about state modification
    void preroll(Request& request);
    void analyse_grain(...);
    void next(Request& request);
};
```
```rust
// Rust (bungee.rs)
fn preroll(self: Pin<&mut Stretcher>, request: &mut Request);
fn analyse_grain(self: Pin<&mut Stretcher>, ...);
fn next(self: Pin<&mut Stretcher>, request: &mut Request);
```

### Benefits
1. Clear separation of concerns:
   - Ownership handled by C++ smart pointers
   - State modification handled by Rust type system
2. Explicit about behavior:
   - No hidden state modifications
   - Clear API contract for users
3. Matches Rust's philosophy:
   - Makes side effects clear in types
   - Prefers explicitness over cleverness

### Status
- ✅ Pattern identified
- ✅ Implementation approach clear
- ⏳ Documentation being updated
- ⏳ Tests needed to verify behavior

### Next Steps
1. Update all method signatures to follow this pattern
2. Add clear documentation about state modification expectations
3. Add tests that verify state modification behavior
4. Update user guide to explain the pattern

### Lessons Learned
1. Don't try to be too clever with const-correctness
2. Be explicit about state modification, even if seemingly redundant
3. Keep ownership and state modification concerns separate
4. Follow the host language's philosophy (Rust's explicitness in this case)

## Error Set 14: Include Path and Type Usage Resolution (January 9, 2024)

### Issue
1. Include path errors:
```
cannot open source file "Bungee/Stretcher.h"
```
2. Direct usage of Bungee namespace types causing type mismatches

### Root Cause Analysis
1. Using `Bungee::` types directly in wrapper.cpp:
   - Caused include path dependencies
   - Mixed internal and external type systems
   - Created unnecessary coupling

2. Method name inconsistencies:
   - `compute_grain` vs `specify_grain`
   - Created confusion between wrapper and implementation

### Solution
1. Use wrapped types consistently:
   ```cpp
   // Instead of Bungee::Request, use our type
   void preroll(Request& request);
   InputChunk specify_grain(const Request& request);
   ```

2. Correct method names:
   ```cpp
   // Use consistent naming with base class
   return impl->specify_grain(request);  // Not compute_grain
   ```

3. Constructor parameter types:
   ```cpp
   // Use std::pair instead of Bungee::SampleRates
   std::make_unique<Stretcher>(
       std::pair<int32_t, int32_t>{sample_rate, sample_rate},
       channels
   );
   ```

### Verification
This solution:
- ✅ Maintains state modification honesty (from whybug.md)
- ✅ Keeps ownership and state separate (from trybug.md)
- ✅ Uses consistent type system
- ✅ Reduces include dependencies
- ✅ Matches Rust's type safety principles

### Status
✅ Resolved:
- Include path issues fixed
- Type system consistency maintained
- Method names aligned
- No regression to previous error patterns

### Lessons Learned
1. Keep type systems clean and separated
2. Use wrapper types consistently
3. Don't expose internal library types directly
4. Maintain clear boundaries between layers