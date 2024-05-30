#![no_std]      // don't link the Rust std lib 
#![no_main]     // disable all Rust-level entry points

use core::convert::TryInto;
use core::panic::PanicInfo;


mod vga_buffer;


static HELLO: &[u8] = b"Hello World!";



// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}



#[no_mangle]    // don't mangle the _start name function
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");
    panic!("Intentional triggering the panic handler...");
    loop {}
}
