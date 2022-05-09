use super::com::ComRegister;
use core::arch::asm;

pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

pub fn inb(source: u16) -> u8 {
    let mut _byte_out = 0;
    unsafe {
        asm!("in al, dx", out("al") _byte_out, in("dx") source, options(nomem, nostack, preserves_flags));
    }
    _byte_out
}

pub fn putchar(port: u16, c: char) {
    outb(port, c as u8);
}

pub fn getchar(port: u16) -> char {
    while !port_read_ready(port) {}

    inb(port) as char
}

const COM_LINE_DATA_READY: u8 = 1 << 0;
fn port_read_ready(port: u16) -> bool {
    inb(port + ComRegister::LineStatus as u16) & COM_LINE_DATA_READY > 0
}
