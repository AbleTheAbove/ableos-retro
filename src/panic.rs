#[cfg(not(test))]
use crate::println;

#[cfg(test)]
use crate::{
    serial_println,
    test::{exit_qemu, QemuExitCode},
};
use core::panic::PanicInfo;
#[cfg(test)]
use lliw::Fg;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

/// Handles panics durring tests
#[cfg(test)]
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}[Failed]{}{}", Fg::Red, Fg::Reset, info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
