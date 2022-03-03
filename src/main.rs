#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(dragon_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use dragon_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    dragon_os::init();

    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    #[cfg(test)]
    test_main();

    panic!();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dragon_os::test_panic_handler(info)
}
