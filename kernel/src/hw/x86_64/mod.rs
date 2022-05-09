use core::arch::asm;

use crate::pmm;

use self::com::ComPort;
use libadamant::{boot::handover::Handover, log, print, println};

pub mod com;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod port;
pub mod stivale2;

const COM_PORT: ComPort = ComPort::COM1;
static mut HANDOVER: Handover = Handover::null();

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

    libadamant::boot::stivale2_to_handover(&stivale_struct, unsafe { &mut HANDOVER });

    for entry in unsafe { HANDOVER.mmap } {
        println!(
            "New entry ! Base = {}, End = {}, Type: {:?}",
            entry.begin, entry.end, entry.mmap_type
        );
    }

    unsafe {
        pmm::pmm_initialize(&HANDOVER.mmap);
    }
    loop {}
}
