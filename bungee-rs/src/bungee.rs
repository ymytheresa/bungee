#[cxx::bridge(namespace = "bungee_rs")]
mod ffi {
    // Shared structs
    #[derive(Debug)]
    struct Request {
        pitch: f64,
        speed: f64,
        position: f64,
    }

    #[derive(Debug)]
    struct InputChunk {
        offset: i32,
        length: i32,
    }

    #[derive(Debug)]
    struct OutputChunk {
        data: Vec<f32>,
        start_request: Request,
        end_request: Request,
    }

    unsafe extern "C++" {
        include!("wrapper.h");

        type Stretcher;

        fn new_stretcher(sample_rate: i32, channels: i32) -> UniquePtr<Stretcher>;
        fn preroll(self: Pin<&mut Stretcher>, request: &mut Request);
        fn specify_grain(self: Pin<&mut Stretcher>, request: &Request) -> InputChunk;
        fn analyse_grain(self: Pin<&mut Stretcher>, data: &[f32], channel_stride: i32);
        fn synthesise_grain(self: Pin<&mut Stretcher>) -> OutputChunk;
        fn next(self: Pin<&mut Stretcher>, request: &mut Request);
    }
}

// Required for opaque C++ type
unsafe impl cxx::ExternType for Stretcher {
    type Id = cxx::type_id!("bungee_rs::Stretcher");
    type Kind = cxx::kind::Opaque;
}

// Re-export the FFI types
pub use ffi::*;