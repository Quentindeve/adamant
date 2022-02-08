pub mod com;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod port;
pub mod stivale2;

/// The x86_64 arch entry point.
extern "C" fn entry_point() -> ! {
    /* gdt::setup_gdt(); */
    /* idt::setup_idt();
    com::init_serial(&com::ComPort::COM2);
    let msg = b"Hello World !";

    for c in msg {
        com::putchar(&com::ComPort::COM2, *c as char);
    } */

    loop {}
}
