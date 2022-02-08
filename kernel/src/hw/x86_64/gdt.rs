//! This is the implementation of the GDT.

use core::mem::size_of;

/// This is the content of the GDTR register.
#[repr(C, packed)]
struct GdtDescriptor {
    size: u16,
    address: u64,
}

impl GdtDescriptor {
    fn new(gdt: &Gdt) -> Self {
        let size = size_of::<Gdt>();
        let address = (gdt as *const Gdt) as u64;

        Self {
            size: size as u16 - 1,
            address,
        }
    }
}

/// Representation of a GDT segment.
struct GdtSegment {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    flags: u8,
    limit_high_and_granularity: u8, // 3-0 are limit_high, 7-4 are granularity
    base_high: u8,
}

impl GdtSegment {
    pub const fn new(base: u32, limit: u32, flags: u8, granularity: u8) -> Self {
        Self {
            limit_low: limit as u16,
            base_low: base as u16,
            base_mid: (base << 16) as u8,
            flags,
            limit_high_and_granularity: ((limit << 24) as u8) | granularity,
            base_high: (base << 24) as u8,
        }
    }
}

/// All flags for a GDT.
const GDT_SEGMENT: u8 = 0b00010000;
const GDT_PRESENT: u8 = 0b10000000;
const GDT_USER: u8 = 0b01100000;
const GDT_EXECUTABLE: u8 = 0b00001000;
const GDT_RW: u8 = 0b00000010;

type Gdt = [GdtSegment; 6];

/// Long-mode granularity is set only when the segment is an executable one.
const LONG_MODE_GRANULARITY: u8 = 0b00100000;

pub const KERNEL_CODE: u16 = 1;
pub const KERNEL_DATA: u16 = 2;
pub const USER_DATA: u16 = 4;
pub const USER_CODE: u16 = 5;

/// Since GDT is always the same despite the context we can let the compile-time do its job.
const GDT: Gdt = [
    GdtSegment::new(0, 0, 0, 0),
    // Kernel Code
    GdtSegment::new(
        0,
        0,
        GDT_PRESENT | GDT_SEGMENT | GDT_EXECUTABLE | GDT_RW,
        LONG_MODE_GRANULARITY,
    ),
    // Kernel Data
    GdtSegment::new(0, 0, GDT_PRESENT | GDT_SEGMENT | GDT_RW, 0),
    GdtSegment::new(0, 0, 0, 0),
    // GDT_USER Data
    GdtSegment::new(0, 0, GDT_PRESENT | GDT_SEGMENT | GDT_RW | GDT_USER, 0),
    GdtSegment::new(
        0,
        0,
        GDT_PRESENT | GDT_SEGMENT | GDT_RW | GDT_EXECUTABLE | GDT_USER,
        LONG_MODE_GRANULARITY,
    ),
];

extern "C" {
    fn gdt_update(descriptor: &GdtDescriptor);
}

/// Loads the GDT.
pub fn setup_gdt() {
    unsafe {
        // Be careful, descriptor goes gulag after gdt_update, maybe we shouldn't
        let descriptor = GdtDescriptor::new(&GDT);
        gdt_update(&descriptor);
    }
}
