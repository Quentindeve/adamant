use core::arch::asm;

use self::com::ComPort;
use libadamant::{log, print, println, boot::handover::Handover};

pub mod com;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod port;
pub mod stivale2;

const COM_PORT: ComPort = ComPort::COM1;

/// The x86_64 arch entry point.
extern "C" fn entry_point(stivale_struct: &stivale_boot::v2::StivaleStruct) -> ! {
    com::init_serial(&COM_PORT);
    log::set_global_logger(log::Logger::new_all(com::log));

    println!("Pre-GDT");
    gdt::setup_gdt();
    println!("Post-GDT");

    println!("Pre-IDT");
    idt::setup_idt();
    println!("Post-IDT");

    unsafe {
        asm!("int 0");
    }

    let mut handover = Handover::null();
    libadamant::boot::stivale2_to_handover(&stivale_struct, &mut handover);

    for entry in handover.mmap {
        println!("New entry ! Base = {}, End = {}, Type: {:?}", entry.begin, entry.end, entry.mmap_type);
    }

    loop {}
}
