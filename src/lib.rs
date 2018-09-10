#![no_std] // don't link the Rust standard library

extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate uart_16550;
extern crate x86_64;
extern crate uefi;
extern crate uefi_logger;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

pub mod gdt;
pub mod serial;

/// Initializes the kernel's logger
pub fn init_logger(st: &'static uefi::table::SystemTable) {
    static mut LOGGER: Option<uefi_logger::Logger> = None;

    let stdout = st.stdout();

    // Construct the logger.
    let logger = unsafe {
        LOGGER = Some(uefi_logger::Logger::new(stdout));

        LOGGER.as_ref().unwrap()
    };

    // Set the logger.
    log::set_logger(logger).unwrap(); // Can only fail if already initialized.

    // Log everything.
    log::set_max_level(log::LevelFilter::Info);

    info!("Started kernel logger");
}

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

#[no_mangle]
pub extern "C" fn __chkstk() {}
