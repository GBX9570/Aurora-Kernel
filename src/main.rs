#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cosmix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use cosmix::println;
use core::panic::PanicInfo;
use x86_64::registers::control::Cr3;
use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::PageTable;
use cosmix::memory::translate_addr;
use cosmix::memory;
use x86_64::{structures::paging::Translate, VirtAddr};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Loading Cosmix....{}", "!");

    cosmix::init();

   let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // new: initialize a mapper
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
    	0xb8000,
    	0x201008,
    	0x0100_0020_1a10,
    	boot_info.physical_memory_offset,
    ]; // same as before

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        // new: use the `mapper.translate_addr` method
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

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
