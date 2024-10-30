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
use cosmix::task::keyboard;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    println!("Welcome to Cosmix!\n");
    cosmix::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

	 allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

	let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("It did not crash!\n");
    cosmix::hlt_loop();
}

async fn async_number () -> u32 {
	42
}

async fn example_task() {
	let number = async_number().await;
	println!("async number: {}\n", number);
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
