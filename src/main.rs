#![no_std]
#![no_main]

use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello World!";

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("Cool Stuff{}", "!?!");

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
