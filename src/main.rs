// src/main.rs

#![no_std]      // don't link the Rust std lib 
#![no_main]     // disable all Rust-level entry points

use core::panic::PanicInfo;
use core::arch::asm;



static HELLO: &[u8] = b"Hello World!";


// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]    // don't mangle the _start name function
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the linker looks for a function
    // named `_start` by default

    // just for me - asm example
    //unsafe { asm!("inc rax"); }

    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }



    loop {}
}
