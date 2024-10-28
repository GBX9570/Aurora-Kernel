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
	use cosmix::memory::BootInfoFrameAllocator;
    use cosmix::memory;
    use x86_64::{structures::paging::Page, VirtAddr}; // new import

	println!("Loading Cosmix....{}", "!");
    cosmix::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
            BootInfoFrameAllocator::init(&boot_info.memory_map)
        };
    
    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    
    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

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
