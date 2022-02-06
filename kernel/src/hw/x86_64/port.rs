use super::com::ComRegister;

extern "C" {
    fn port_out(dest: u16, value: u8);
    fn port_in(source: u16) -> u8;
}

pub fn outb(port: u16, value: u8) {
    let dest = port;
    unsafe {
        port_out(dest, value);
    }
}

const COM_LINE_DATA_READY: u8 = 1 << 0;
fn port_read_ready(port: u16) -> bool {
    unsafe { return (port_in(port + ComRegister::LineStatus as u16) & COM_LINE_DATA_READY) > 0 }
}

pub fn inb(port: u16) -> u8 {
    while !port_read_ready(port) {}

    unsafe { port_in(port) }
}
