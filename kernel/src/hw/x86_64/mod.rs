use core::arch::asm;

use self::com::ComPort;

pub mod com;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod port;
pub mod stivale2;

/// The x86_64 arch entry point.
extern "C" fn entry_point(_stivale_struct: &stivale_boot::v2::StivaleStruct) -> ! {
    com::init_serial(&com::ComPort::COM1);

    com::write_text(ComPort::COM1, "Pre GDT");
    gdt::setup_gdt();
    com::write_text(ComPort::COM1, "Post GDT");

    com::write_text(ComPort::COM1, "Pre IDT");
    idt::setup_idt();
    com::write_text(ComPort::COM1, "Post IDT");

    unsafe {
        asm!("int 0");
    }
    loop {}
}
