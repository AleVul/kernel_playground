// std is OS specific so it does not work here.
#![no_std]
// To avoid `crt0` and `start` language item.
// Since we dont run over an OS but implement one.
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello world!";

/// On linux, the linker looks for a function
/// named `_start` to use as an entry point. So
/// this is what we will use.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// Panic handler, will be called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
