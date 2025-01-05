# Known Bugs and Solutions

## FFI Array Move Error (2024-01-05)

### Error
When implementing the `synthesise_grain` method in the FFI wrapper, encountered move errors with array elements:

```rust
error[E0508]: cannot move out of type `[Box<Bungee_Request>; 2]`, a non-copy array
   --> src/lib.rs:161:32
    |
161 |         output.begin_request = (*ffi_requests[0]).into();
    |                                ^^^^^^^^^^^^^^^^^^
    |                                |
    |                                cannot move out of here
    |                                move occurs because `*ffi_requests[_]` has type `Bungee_Request`, which does not implement the `Copy` trait
```

### Root Cause
- The FFI structs didn't implement `Copy` trait
- Attempting to move values out of array elements which isn't allowed in Rust
- Using heap allocation (`Box`) unnecessarily complicated the ownership

### Solution
1. Added `Clone` and `Copy` traits to FFI structs since they only contain primitive types:
```rust
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bungee_Request {
    pub position: f64,
    pub speed: f64,
    pub pitch: f64,
    pub reset: bool,
}
```

2. Simplified `synthesise_grain` to use stack allocation and references:
```rust
// Create FFI requests using Copy trait
let ffi_begin_request = ffi::Bungee_Request::from(output.begin_request.clone());
let ffi_end_request = ffi::Bungee_Request::from(output.end_request.clone());

let mut ffi_chunk = ffi::Bungee_OutputChunk {
    data: output.data.as_mut_ptr(),
    frame_count: output.frame_count,
    channel_stride: output.channel_stride,
    request: [&mut ffi_begin_request, &mut ffi_end_request],
};
```

### Lessons Learned
1. For FFI structs containing only primitive types, implementing `Copy` is cleaner than managing heap allocations
2. When working with arrays in Rust, prefer using references or `Copy` types to avoid move complications
3. Stack allocation is often simpler and more efficient than heap allocation for small, fixed-size data

### Related Issues
- C++ compilation warnings about treating C input as C++ (non-critical)
- Need to verify FFI struct layout matches C++ side 