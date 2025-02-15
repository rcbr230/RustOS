// main.rs

// needs to not try and use the linker for windows
// create that with the target being bare metal:
//      rustup target add thumbv7em-none-eabihf
// run by changing the target to the one above
//      cargo build --target thumbv7em-none-eabihf

// disable standard library. Not using any OS calls since we are 
//  making our own OS
#![no_std] // disable standard library
#![no_main] // disable main fn requirement

/* Rust is missing a panic function
 *  Readd that here. See link to read on what panic is
 * 
 [profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
 is added to the .toml file so that when a panic happens, instead of unwinding(complicated), we abort completely
 * https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html
 */ 
 // PanicInfo contains the file and line where a panic happened
 // ! means that the function will never return. ! is a "never" type
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// use no_mangle so that name stays as _start rather than some insane giberish
// create a new _start fn to replace main
// 'extern "C"' tells compiler to use C calling convention instead of an unspecfied rust calling convention.
// this is not called by any function, so it should never return.

// u8 - unsigned 8-bit int (1 byte with no sign bit)
// store 'Hello World!' as byte code
static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    // create a mutable pointer with the value 0xb8000 as an 8 bit un signed int
    let vga_buffer = 0xb8000 as *mut u8;

    // iterate over static variable 
    for (i, &byte) in HELLO.iter().enumerate() {
        // unsafe because we are dereferencing vga_buffer
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color, 0xb is cyan
        }
    }

    loop {}
}