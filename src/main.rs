#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("Hello World{}", "!");
    println!("Hello World{}", "!");
    println!("Hello World{}", "!");
    println!("Hello World{}", "!");
    print!("Hello World{}", "!");
    println!();
    print!("Hello World{}", "!");
    println!();
    print!("Hello World");
    loop {}
}