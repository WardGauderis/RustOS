#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

use os::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
	println!("Hello World{}", "!");

	os::init();

	use x86_64::registers::control::Cr3;

	let (level_4_page_table, _) = Cr3::read();
	println!("at: {:?}", level_4_page_table.start_address());

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
