#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate blog_os;
extern crate uefi;

use blog_os::exit_qemu;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn uefi_start(_: uefi::Handle, st: &'static uefi::table::SystemTable) -> ! {
    blog_os::init_logger(st);
    panic!();
}

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    serial_println!("ok");

    unsafe {
        exit_qemu();
    }
    loop {}
}
