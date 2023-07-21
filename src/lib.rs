#![no_std]

extern crate cstr_core;

use cstr_core::cstr;
use cstr_core::c_char;

type Callback = unsafe extern fn(*const c_char);

#[no_mangle]
pub unsafe extern fn get_string_in_callback(callback: Callback) {
    let string = cstr!("Hello from Rust!");
    // as_ptr() keeps ownership in rust unlike into_raw()
    callback(string.as_ptr())
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
