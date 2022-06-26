#![no_main]
#![no_std]
#![feature(rustc_private)]

extern crate libc;

extern "C" {
    fn ShowCrashReports();
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        ShowCrashReports();
        let mut buffer = libc::malloc(13);
        const c_str: &'static str = "Hello\0";
        libc::strcpy(buffer as *mut i8, c_str.as_ptr() as *const i8);
        libc::free(buffer);
        libc::printf(buffer as *const _);
    }
    0
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
