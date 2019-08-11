// std is OS specific so it does not work here.
#![no_std]
// To avoid `crt0` and `start` language item.
// Since we dont run over an OS but implement one.
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

/// On linux, the linker looks for a function
/// named `_start` to use as an entry point. So
/// this is what we will use.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("Foo bar");

    loop {}
}

/// Panic handler, will be called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
