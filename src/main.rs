#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cosmix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use cosmix::println;
use core::panic::PanicInfo;
use cosmix::print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Loading Cosmix....{}", "!");

    cosmix::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }
		
    // uncomment line below to trigger a stack overflow
    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("Not Crashed Yet");
    cosmix::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    cosmix::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cosmix::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
