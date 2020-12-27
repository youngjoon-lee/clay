#![no_std]  // don't link the Rust standard library
#![no_main]  // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

// Called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {  // returning the 'never' type
    println!("{}", info);
    loop{}
}

// Overwrite the entry point
#[no_mangle] // disabling the name mangling to ensure that Rust compiler really outputs a func name with the name `_start`.
pub extern "C" fn _start() -> ! {
    // This is the entry point, since the linker looks for a function named `_start` by default.

    println!("Hello World{}", "!");

    loop {}
}
