#![feature(abi_x86_interrupt)]
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate blog_os;
extern crate x86_64;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate uefi;

use core::panic::PanicInfo;

/// This function is the entry point, since the linker looks for a function
/// named `uefi_start` by default.
#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn uefi_start(_: uefi::Handle, st: &'static uefi::table::SystemTable) -> ! {
    blog_os::init_logger(st);

    info!("Hello {}!", "World");

    blog_os::gdt::init();
    init_idt();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    error!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    loop {}
}

use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(blog_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    info!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
    error!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}
