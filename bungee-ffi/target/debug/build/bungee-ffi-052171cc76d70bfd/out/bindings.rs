/* automatically generated by rust-bindgen 0.69.5 */

pub const bungee_error_BUNGEE_OK: bungee_error = 0;
pub const bungee_error_BUNGEE_NULL_POINTER: bungee_error = 1;
pub const bungee_error_BUNGEE_INVALID_PARAM: bungee_error = 2;
pub const bungee_error_BUNGEE_MEMORY: bungee_error = 3;
pub const bungee_error_BUNGEE_INVALID_STATE: bungee_error = 4;
pub const bungee_error_BUNGEE_BUFFER_TOO_SMALL: bungee_error = 5;
pub type bungee_error = ::std::os::raw::c_uint;
pub use self::bungee_error as bungee_error_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bungee_sample_rates_t {
    pub input_rate: ::std::os::raw::c_int,
    pub output_rate: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_bungee_sample_rates_t() {
    const UNINIT: ::std::mem::MaybeUninit<bungee_sample_rates_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<bungee_sample_rates_t>(),
        8usize,
        concat!("Size of: ", stringify!(bungee_sample_rates_t))
    );
    assert_eq!(
        ::std::mem::align_of::<bungee_sample_rates_t>(),
        4usize,
        concat!("Alignment of ", stringify!(bungee_sample_rates_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).input_rate) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bungee_sample_rates_t),
            "::",
            stringify!(input_rate)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).output_rate) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bungee_sample_rates_t),
            "::",
            stringify!(output_rate)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bungee_request_t {
    pub position: f64,
    pub speed: f64,
    pub pitch: f64,
    pub reset: bool,
}
#[test]
fn bindgen_test_layout_bungee_request_t() {
    const UNINIT: ::std::mem::MaybeUninit<bungee_request_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<bungee_request_t>(),
        32usize,
        concat!("Size of: ", stringify!(bungee_request_t))
    );
    assert_eq!(
        ::std::mem::align_of::<bungee_request_t>(),
        8usize,
        concat!("Alignment of ", stringify!(bungee_request_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).position) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bungee_request_t),
            "::",
            stringify!(position)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).speed) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bungee_request_t),
            "::",
            stringify!(speed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pitch) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(bungee_request_t),
            "::",
            stringify!(pitch)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reset) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(bungee_request_t),
            "::",
            stringify!(reset)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bungee_input_chunk_t {
    pub begin: i32,
    pub end: i32,
}
#[test]
fn bindgen_test_layout_bungee_input_chunk_t() {
    const UNINIT: ::std::mem::MaybeUninit<bungee_input_chunk_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<bungee_input_chunk_t>(),
        8usize,
        concat!("Size of: ", stringify!(bungee_input_chunk_t))
    );
    assert_eq!(
        ::std::mem::align_of::<bungee_input_chunk_t>(),
        4usize,
        concat!("Alignment of ", stringify!(bungee_input_chunk_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).begin) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bungee_input_chunk_t),
            "::",
            stringify!(begin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).end) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bungee_input_chunk_t),
            "::",
            stringify!(end)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bungee_output_chunk_t {
    pub data: *mut f32,
    pub frame_count: i32,
    pub channel_stride: usize,
}
#[test]
fn bindgen_test_layout_bungee_output_chunk_t() {
    const UNINIT: ::std::mem::MaybeUninit<bungee_output_chunk_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<bungee_output_chunk_t>(),
        24usize,
        concat!("Size of: ", stringify!(bungee_output_chunk_t))
    );
    assert_eq!(
        ::std::mem::align_of::<bungee_output_chunk_t>(),
        8usize,
        concat!("Alignment of ", stringify!(bungee_output_chunk_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bungee_output_chunk_t),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frame_count) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bungee_output_chunk_t),
            "::",
            stringify!(frame_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).channel_stride) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(bungee_output_chunk_t),
            "::",
            stringify!(channel_stride)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bungee_stretcher {
    _unused: [u8; 0],
}
pub type bungee_stretcher_t = bungee_stretcher;
extern "C" {
    pub fn bungee_init() -> bungee_error_t;
}
extern "C" {
    pub fn bungee_cleanup();
}
extern "C" {
    pub fn bungee_create(
        rates: bungee_sample_rates_t,
        channels: ::std::os::raw::c_int,
    ) -> *mut bungee_stretcher_t;
}
extern "C" {
    pub fn bungee_destroy(stretcher: *mut bungee_stretcher_t);
}
extern "C" {
    pub fn bungee_preroll(
        stretcher: *mut bungee_stretcher_t,
        request: *const bungee_request_t,
    ) -> bungee_error_t;
}
extern "C" {
    pub fn bungee_specify_grain(
        stretcher: *mut bungee_stretcher_t,
        input_data: *const f32,
        frame_count: usize,
        chunk: *mut bungee_input_chunk_t,
    ) -> bungee_error_t;
}
extern "C" {
    pub fn bungee_analyse_grain(
        stretcher: *mut bungee_stretcher_t,
        input_data: *const f32,
        channel_stride: usize,
    ) -> bungee_error_t;
}
extern "C" {
    pub fn bungee_synthesise_grain(
        stretcher: *mut bungee_stretcher_t,
        chunk: *mut bungee_output_chunk_t,
    ) -> bungee_error_t;
}
extern "C" {
    pub fn bungee_next(
        stretcher: *mut bungee_stretcher_t,
        request: *mut bungee_request_t,
    ) -> bungee_error_t;
}
extern "C" {
    pub fn bungee_is_flushed(stretcher: *const bungee_stretcher_t) -> bool;
}
extern "C" {
    pub fn bungee_max_input_frame_count(stretcher: *const bungee_stretcher_t) -> usize;
}
