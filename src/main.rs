// std is OS specific so it does not work here.
#![no_std]
// To avoid `crt0` and `start` language item.
// Since we dont run over an OS but implement one.
#![no_main]
// for custom testing framework
#![feature(custom_test_frameworks)]
#![test_runner(kernel_playground::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel_playground::println;

/// On linux, the linker looks for a function
/// named `_start` to use as an entry point. So
/// this is what we will use.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // For unit tests
    #[cfg(test)]
    test_main();

    loop {}
}

/// Panic handler, will be called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// Panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel_playground::test_panic_handler(info)
}
