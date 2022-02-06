//! This file contains a little COM """""driver"""""

use super::port::*;

/// Represents all COM ports
#[repr(u16)]
#[derive(Copy, Clone)]
pub enum ComPort {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
}

/// Represents all COM registers
#[repr(u16)]
pub enum ComRegister {
    Data = 0,
    InterruptOrBaudHigh = 1,
    InterruptIdOrFIFO = 2,
    LineControl = 3,
    ModemControlOrIrq = 4,
    LineStatus = 5,
    ModemStatus = 6,
    ScratchRegister = 7,
}

pub enum ComStatus {
    Ok,
    Faulty,
}

/// This functions initializes a serial port
pub fn init_serial(com: &ComPort) -> ComStatus {
    let com = *com as u16;
    outb(com + ComRegister::InterruptOrBaudHigh as u16, 0x00);
    outb(com + ComRegister::LineControl as u16, 0x80);
    outb(com + ComRegister::Data as u16, (115200 / 9600) as u8); // Je sais pas comment fonctionne le diviseur ?
    outb(com + ComRegister::InterruptOrBaudHigh as u16, 0); // Same ?
    outb(com + ComRegister::LineControl as u16, 0x03);
    outb(com + ComRegister::InterruptIdOrFIFO as u16, 0xC7);
    outb(com + ComRegister::ModemControlOrIrq as u16, 0x0B);
    outb(com + ComRegister::ModemControlOrIrq as u16, 0x1E);
    outb(com + ComRegister::Data as u16, 0xAE);

    if inb(com) != 0xAE {
        return ComStatus::Faulty;
    } else {
        outb(com + ComRegister::ModemControlOrIrq as u16, 0x0F);
        return ComStatus::Ok;
    }
}

/// Puts a char to the specified COM port.
pub fn putchar(com: &ComPort, c: char) {
    outb(*com as u16, c as u8);
}

/// Gets a char from the specified COM port.
/// Be aware that this function loops until data is available.
pub fn getchar(com: &ComPort) -> u8 {
    inb(*com as u16)
}
