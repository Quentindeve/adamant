use core::default;

pub const HANDOVER_TAG: u64 = 0x68616e646f766572;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum HandoverMmapType {
    MmapFree = 0,
    MmapUsed = 1,
    MmapReclaimable = 2,

    MmapReserved = 3,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HandoverMmapEntry {
    pub begin: usize,
    pub end: usize,
    pub mmap_type: HandoverMmapType,
}

impl HandoverMmapEntry {
    pub fn null() -> Self {
        Self {
            begin: 0,
            end: 0,
            mmap_type: HandoverMmapType::MmapUsed,
        }
    }
}

pub const HANDOVER_MMAP_MAX_SIZE: usize = 64;

#[repr(C)]
pub struct HandoverMmap {
    pub size: usize,
    pub entries: [HandoverMmapEntry; HANDOVER_MMAP_MAX_SIZE],
}

#[repr(C)]
pub struct HandoverFramebuffer {
    pub present: bool,
    pub address: *mut u8,

    pub width: u32,
    pub height: u32,

    pub pitch: u32,
    pub bpp: u32,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct HandoverModule {
    pub size: usize,
    pub address: usize,
    pub module_name: [libc::c_char; 32],
}

pub const MAX_MODULE_COUNT: usize = 16;

#[repr(C)]
pub struct HandoverModules {
    pub size: usize,
    pub modules: [HandoverModule; MAX_MODULE_COUNT],
}

impl HandoverModules {
    pub fn empty() -> HandoverModules {
        HandoverModules {
            size: 0,
            modules: [HandoverModule::default(); MAX_MODULE_COUNT],
        }
    }
}

#[repr(C)]
pub struct HandoverCmdLines {
    pub present: bool,
    pub cmd_line: [char; 32],
}

#[repr(C)]
pub struct Handover {
    pub tag: u64,

    pub mmap: HandoverMmap,
    pub framebuffer: HandoverFramebuffer,
    pub modules: HandoverModules,

    pub rsdp: usize,
    pub fdt: usize,

    pub cmd_lines: HandoverCmdLines,
}
