use core::arch::asm;

pub mod com;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod port;
pub mod stivale2;

/// The x86_64 arch entry point.
extern "C" fn entry_point(_stivale_struct: &stivale_boot::v2::StivaleStruct) -> ! {
    gdt::setup_gdt();
    idt::setup_idt();
    com::init_serial(&com::ComPort::COM1);

    let msg = b"Hello World !";
    for c in msg {
        com::putchar(&com::ComPort::COM1, *c as char);
    }

    unsafe {
        asm!("int 0");
    }
    loop {}
}
