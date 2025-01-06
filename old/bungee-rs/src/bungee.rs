#[cxx::bridge(namespace = "bungee_rs")]
mod ffi {
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
        fn specify_grain(self: Pin<&Stretcher>, request: &Request) -> InputChunk;
        fn analyse_grain(self: Pin<&Stretcher>, data: &[f32], channel_stride: i32);
        fn synthesise_grain(self: Pin<&Stretcher>) -> OutputChunk;
        fn next(self: Pin<&mut Stretcher>, request: &mut Request);
    }
} 