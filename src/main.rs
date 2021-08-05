#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;


use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use x86_64::{
	VirtAddr,
};

use os::{
	allocator, memory,
	memory::BootInfoFrameAllocator,
	println,
	task::{executor::Executor, keyboard, Task},
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
	println!("Hello World{}", "!");
	os::init();

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

	async fn async_number() -> u32 { 42 }

	async fn example_task() {
		let number = async_number().await;
		println!("async number: {}", number);
	}

	let mut executor = Executor::new();
	executor.spawn(Task::new(example_task()));
	executor.spawn(Task::new(keyboard::print_keypresses()));
	executor.run();

	#[cfg(test)]
	test_main();

	println!("It did not crash!");
	os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	println!("{}", _info);
	os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { os::test_panic_handler(info) }

#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}
