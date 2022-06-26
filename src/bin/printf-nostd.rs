#![no_main]
#![no_std]
#![feature(rustc_private)]

extern crate libc;

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Since we are passing a C string the final null character is mandatory
    const HELLO: &'static str = "Hello, world! %d + %d = %d\n\0";
    let x: i32 = 1;
    let y: i32 = 2;
    unsafe {
        libc::printf(HELLO.as_ptr() as *const _, x, y, x+y);
    }
    0
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
