#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use x86_64::structures::idt::ExceptionVector::Page;
use x86_64::structures::paging::Translate;
use blog_os::{memory, println};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    //map an unused page
    let page  = Page::containing_address(VirtAddr(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

   let page_prt: *mut u64 = page.start_address().as_mut_prt();
    unsafe { page_prt.offset(400).write_volatile(0x_f021_f077_f065_f04e)}

    #[cfg(test)]
    test_main();

    println!("It dit not crash!");
    blog_os::htl_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::htl_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
