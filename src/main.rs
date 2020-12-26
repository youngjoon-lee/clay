#![no_std]  // don't link the Rust standard library
#![no_main]  // disable all Rust-level entry points

use core::panic::PanicInfo;

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // returning the 'never' type
    loop{}
}

static HELLO: &[u8] = b"Hello World!";

// Overwrite the entry point
#[no_mangle] // disabling the name mangling to ensure that Rust compiler really outputs a func name with the name `_start`.
pub extern "C" fn _start() -> ! {
    // This is the entry point, since the linker looks for a function named `_start` by default.

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
