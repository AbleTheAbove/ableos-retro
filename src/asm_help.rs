pub fn pain() -> u64 {
    let x: u64;
    unsafe {
        asm!("rdrand {}", out(reg) x);
    }
    x
}
