#![no_std]
#![no_main]  // not to use the Rust runtime and crt0

use core::panic::PanicInfo;

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // returning the 'never' type
    loop{}
}

// Overwrite the entry point
#[no_mangle] // disabling the name mangling to ensure that Rust compiler really outputs a func name with the name `_start`.
pub extern "C" fn _start() -> ! {
    loop{}
}
