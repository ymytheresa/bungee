# The State Modification Dilemma in Bungee Wrapper

## Core Problem: Honesty in Type Systems

The fundamental challenge in the Bungee Rust wrapper isn't just a technical issue - it's about truthfully representing behavior in the type system. This document explains why this is so important and why it's been challenging to solve.

### The Reality of the Code

The Bungee C++ library is fundamentally stateful:
```cpp
class Stretcher {
    // Internal state
    Buffer grainBuffer;
    float* processedData;
    // ... more state ...

    void specify_grain(Request& req) {
        // MODIFIES internal state
        this->grainBuffer.update();
        this->processedData = process(req);
    }
};
```

Every major operation modifies internal state:
- `preroll`: Updates internal timing state
- `specify_grain`: Updates grain buffers
- `analyse_grain`: Processes and stores audio data
- `synthesise_grain`: Generates output based on state
- `next`: Updates timing and request state

### The Type System Conflict

This creates a three-way conflict between:

1. **C++ Const Correctness**:
   ```cpp
   // Can't be const because it modifies state
   void specify_grain(Request& req);
   
   // Making it const would be a lie
   void specify_grain(Request& req) const;  // 🚫 Lies!
   ```

2. **Rust's Mutability Rules**:
   ```rust
   // Doesn't show that state is modified
   fn specify_grain(&self, req: &Request);  // 🚫 Lies!
   
   // Tells the truth about mutation
   fn specify_grain(self: Pin<&mut Self>, req: &Request);  // ✅ Truth
   ```

3. **CXX Bridge Requirements**:
   - Needs to maintain safety guarantees
   - Must accurately represent C++ behavior
   - Has to follow Rust's ownership rules

### Failed Solution Attempts

1. **Making Methods Const** ❌
   ```cpp
   // Tried making methods const
   void specify_grain(Request& req) const;
   ```
   Problem: Lies about state modification

2. **Using Shared Pointers** ❌
   ```cpp
   std::shared_ptr<Stretcher> impl;
   ```
   Problem: Solves ownership but not state modification

3. **Removing Pin<&mut>** ❌
   ```rust
   fn specify_grain(&self, req: &Request);
   ```
   Problem: Hides state modification from Rust

### The Correct Solution: Tell the Truth

The solution is to be honest about state modification:

1. **In C++**: Methods are non-const because they modify state
   ```cpp
   class Stretcher {
       void specify_grain(Request& req);  // Clearly modifies state
   };
   ```

2. **In Rust**: Use `Pin<&mut>` to show state modification
   ```rust
   fn specify_grain(self: Pin<&mut Stretcher>, req: &Request);
   ```

3. **Documentation**: Be explicit about behavior
   ```rust
   /// This method modifies internal state:
   /// - Updates grain buffers
   /// - Processes audio data
   /// - Changes timing information
   fn specify_grain(self: Pin<&mut Stretcher>, req: &Request);
   ```

### Why This Matters

1. **Safety**: 
   - Rust's safety guarantees depend on accurate type information
   - Hiding state modification can lead to race conditions
   - Incorrect const-ness can lead to undefined behavior

2. **Maintainability**:
   - Clear types make code behavior obvious
   - Future developers know what to expect
   - Easier to reason about thread safety

3. **Correctness**:
   - The type system should reflect reality
   - If code modifies state, types should show it
   - No hidden side effects

### Lessons Learned

1. **Be Honest in Types**:
   - Don't try to hide state modification
   - Let the type system show the truth
   - Make behavior explicit

2. **Respect Language Philosophy**:
   - Rust values explicitness
   - C++ values const correctness
   - Follow each language's principles

3. **Separate Concerns**:
   - Ownership (shared_ptr) is different from state modification (Pin<&mut>)
   - Don't try to solve one with the other
   - Keep responsibilities clear

### Moving Forward

This understanding guides our implementation:
1. Use `Pin<&mut>` consistently for state modification
2. Keep methods non-const in C++
3. Document state changes clearly
4. Use shared_ptr only for ownership
5. Make behavior explicit in types

The solution isn't about finding clever hacks - it's about being honest in our type signatures about what our code actually does. 