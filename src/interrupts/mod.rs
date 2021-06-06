use crate::{gdt, info};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

/// Module for APIC
pub mod apic;
/// Module for PIC
pub mod pic;

/// Note what all the interrupts are
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    /// Timer offset
    Timer = pic::PIC_1_OFFSET,
    /// Keyboard offset
    Keyboard,
    /// Mouse offset
    Mouse = 44,
}


impl Into<u8> for InterruptIndex{
    fn into(self) -> u8 {
        self as u8
    }
}
impl Into<usize> for InterruptIndex{
    fn into(self) -> usize {
        self as usize
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.into()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.into()].set_handler_fn(keyboard_interrupt_handler);
        idt[InterruptIndex::Mouse.into()].set_handler_fn(crate::drivers::mouse::mouse_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt
    };
}

/// Initialize the IDT table
pub fn init_idt() {
    IDT.load();
}

use crate::hlt_loop;
use x86_64::structures::idt::PageFaultErrorCode;

extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    // use x86_64::registers::control::Cr2;
    exception();
    // println!(": PAGE FAULT");
    // println!("Accessed Address: {:?}", Cr2::read());
    // println!("Error Code: {:?}", error_code);
    // println!("{:#?}", stack_frame);
    hlt_loop();
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    exception();
    // println!(": BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    exception();
    panic!("DOUBLE FAULT\n{:#?}", stack_frame);
}
// DEPRECATE: unused, likely to not be used
fn exception() {
    // util::term_set(Red);
    // print!("EXCEPTION ");
    // util::term_reset();
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
// TODO: Move to pic
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {

    unsafe {
        pic::PICS
            .lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.into());
    }
}

/// TODO: Move to a keyboard module
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode); // new

    unsafe {
        pic::PICS
            .lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.into());
    }
}
