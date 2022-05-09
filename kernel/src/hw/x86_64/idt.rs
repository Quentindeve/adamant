use core::mem::size_of;

use super::gdt::KERNEL_CODE;

#[repr(C, packed)]
struct IdtDescriptor {
    size: u16,
    addr: u64,
}

impl IdtDescriptor {
    fn new(idt: &Idt) -> Self {
        Self {
            size: (size_of::<Idt>() - 1) as u16,
            addr: (idt as *const _) as u64,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,
    code_segment: u16,
    ist: u8,
    attr: u8,
    offset_mid: u16,
    offset_high: u32,
    _zero: u32,
}

impl IdtEntry {
    const fn empty() -> Self {
        Self {
            offset_low: 0,
            code_segment: 0,
            ist: 0,
            attr: 0,
            offset_mid: 0,
            offset_high: 0,
            _zero: 0,
        }
    }

    const fn new(handler: u64, code_index: u16, attr: u8) -> Self {
        Self {
            offset_low: handler as u16,
            code_segment: code_index,
            ist: 0,
            attr,
            offset_mid: (handler >> 16) as u16,
            offset_high: (handler >> 32) as u32,
            _zero: 0,
        }
    }
}

#[allow(dead_code)]
#[repr(u8)]
enum InterruptType {
    Trap = 0xEF,
    User = 0x60,
    Gate = 0x8E,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtAttributes(u8);

#[link(name = "adamant-x86_64")]
extern "C" {
    static __interrupt_vector: [u64; 256];
    fn load_idt(idt_descriptor: &IdtDescriptor);
}

pub const IDT_LENGTH: usize = 256;
type Idt = [IdtEntry; IDT_LENGTH];

static mut IDT: Idt = [IdtEntry::empty(); IDT_LENGTH];

#[link(name = "adamant-x86_64")]
extern "C" {
    fn disable_interrupts();
    fn enable_interrupts();
}

fn init_idt() {
    for i in 0..IDT_LENGTH {
        unsafe {
            IDT[i] = IdtEntry::new(
                __interrupt_vector[i],
                KERNEL_CODE * 8,
                InterruptType::Gate as u8,
            );
        }
    }
}

pub fn setup_idt() {
    unsafe {
        disable_interrupts();
        init_idt();

        // Be careful, descriptor goes gulag after load_idt, maybe we shouldn't
        let descriptor = IdtDescriptor::new(&IDT);

        load_idt(&descriptor);
        enable_interrupts();
    }
}
