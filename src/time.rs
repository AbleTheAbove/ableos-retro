// Time
// NOTE: Using as a guide https://wiki.osdev.org/CMOS#Reading_All_RTC_Time_and_Date_Registers
extern crate cpuio;

use cpuio::{inb, outb, Port};

pub static CMOS_PORT: u8 = 0x70;
pub static CMOS_DATA_PORT: u8 = 0x71;
#[repr(u16)]
pub enum RtcRegisters {
    Second = 0x00,
    Minute = 0x02,
    Hour = 0x04,
    Day = 0x07,
    Month = 0x08,
    Year = 0x09,
    // Century,
}

fn get_update_in_progress_flag() -> u8 {
    let x;
    unsafe {
        outb(CMOS_PORT, 0x0A);
        x = (inb(CMOS_DATA_PORT.into()) & 0x80);
    }
    return x;
}
// TODO
pub fn get_rtc_register(reg: u16) -> u8 {
    let x;
    unsafe {
        outb(CMOS_PORT, reg);
        x = (inb(CMOS_DATA_PORT.into()) & 0x80);
    }
    return x;
}

pub struct DateTime {
    second: u8,             /* seconds,  range 0 to 59          */
    minute: u32,            /* minutes, range 0 to 59           */
    hour: u32,              /* hours, range 0 to 23             */
    mday: u32,              /* day of the month, range 1 to 31  */
    mon: u32,               /* month, range 0 to 11             */
    year: u32,              /* The number of years since 1900   */
    wday: u32,              /* day of the week, range 0 to 6    */
    yday: u32,              /* day in the year, range 0 to 365  */
    daylight_savings: bool, /* daylight saving time             */
}
impl DateTime {
    fn update(&mut self) {
        self.second = get_rtc_register(RtcRegisters::Second as u16);
    }
}
