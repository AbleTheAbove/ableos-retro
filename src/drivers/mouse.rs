use crate::{info, success};
use crate::ps2_mouse::{Mouse, MouseState};
use spinning_top::Spinlock;
use spin::Mutex;
use x86_64::instructions::port::PortReadOnly;
use x86_64::structures::idt::InterruptStackFrame;
use crate::interrupts::pic::PICS;
use crate::interrupts::InterruptIndex;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref MOUSE: Mutex<Mouse> = Mutex::new(Mouse::new());
}

// Initialize the mouse and set the on complete event.
pub(crate) fn init_mouse() {
    let mut mouse =MOUSE.lock();
    info!("Trying to initialize mouse");
    mouse.init().unwrap();
    mouse.set_on_complete(on_complete);
    success!("Mouse initialized");
}

// This will be fired when a packet is finished being processed.
fn on_complete(mouse_state: MouseState) {
    info!("{:?}", mouse_state);
}

// An example interrupt based on https://os.phil-opp.com/hardware-interrupts/.
// The ps2 mouse is configured to fire
// interrupts at PIC offset 12.
extern "x86-interrupt" fn mouse_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    let mut port = PortReadOnly::new(0x60);
    let packet = unsafe { port.read() };
    MOUSE.lock().process_packet(packet);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Mouse.into());
    }
}
