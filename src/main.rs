#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cosmix::test_runner)]
#![reexport_test_harness_main = "test_main"]
use cosmix::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
extern crate alloc;
use cosmix::allocator;
use cosmix::memory::{self, BootInfoFrameAllocator};
use x86_64::VirtAddr;
use cosmix::task::{Task, simple_executor::SimpleExecutor};
use cosmix::error;
use cosmix::warn;
use cosmix::info;
mod term;
use x86_64::instructions::port::Port;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    println!("Welcome to Cosmix!\n");
    cosmix::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

	 allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

	info!("Cosmix Has Loaded Successfully!\n");
    info!("System Tooltips appear to be functioning. (or not if you can't see this lmao)\n");
    warn!("This project is provided with ABSOLUTELY NO WARRANTY. If something breaks, YOU'RE responsible in the eyes of the law - by using this software you agree to this and the GPL 3.0 license agreement.\n");
    println!("Blame yourself, because Cosmix is stable from when you use it for the first time.\n");
    error!("The Terminal Shell could not be loaded, reason: does not exist right now sorry guys\n");

    #[cfg(test)]
    test_main();
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
