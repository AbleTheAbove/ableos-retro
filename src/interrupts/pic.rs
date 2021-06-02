use pic8259::ChainedPics;
use spin;
/// PIC 1 offset
pub const PIC_1_OFFSET: u8 = 32;
/// PIC 2 Offset
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
/// PIC reference
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
