//! This file describes all structures of the Handover boot protocol, first defined [here](https://github.com/brutal-org/brutal/blob/main/sources/libs/bal/boot/handover.h) by BRUTAL developers.

/// The Handover's magic, "handover" in ASCII.
pub const HANDOVER_TAG: u64 = 0x68616e646f766572;

/// Handover memory map entry types.
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum HandoverMmapType {
    /// This entry is free, can be used.
    MmapFree = 0,
    /// This entry contains kernel, modules or other things unmoveable, cannot be used.
    MmapUsed = 1,
    /// This entry contained something not used anymore like bootloader, can be reclaimed.
    MmapReclaimable = 2,
    /// This entry is reserved.
    MmapReserved = 3,
}

/// Handover memory map entry.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HandoverMmapEntry {
    /// First address.
    pub begin: usize,
    /// Last address.
    pub end: usize,
    /// Type of the entry.
    pub mmap_type: HandoverMmapType,
}

impl HandoverMmapEntry {
    /// Returns a null unusable entry.
    pub const fn null() -> Self {
        Self {
            begin: 0,
            end: 0,
            mmap_type: HandoverMmapType::MmapUsed,
        }
    }

    pub const fn length(&self) -> usize {
        self.end - self.begin - 1 // [begin; end[
    }
}

/// The max size of the Handover's memory map.
pub const HANDOVER_MMAP_MAX_SIZE: usize = 64;

/// Handover's memory map.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HandoverMmap {
    /// The size of the memory map. Even if the entries field is always initialized at maximum, isn't mandatory to fill all.
    pub size: usize,
    /// Entries of the memory map.
    pub entries: [HandoverMmapEntry; HANDOVER_MMAP_MAX_SIZE],
}

impl HandoverMmap {
    /// Returns an empty memory map.
    pub const fn empty() -> Self {
        Self {
            size: 0,
            entries: [HandoverMmapEntry::null(); HANDOVER_MMAP_MAX_SIZE],
        }
    }
}

/// Structure representing an iterator on a `HandoverMmap`
pub struct HandoverMmapIterator {
    count: usize,
    mmap: HandoverMmap,
}

impl IntoIterator for HandoverMmap {
    type Item = HandoverMmapEntry;

    type IntoIter = HandoverMmapIterator;

    fn into_iter(self) -> Self::IntoIter {
        HandoverMmapIterator {
            count: 0,
            mmap: self.clone(),
        }
    }
}

impl Iterator for HandoverMmapIterator {
    type Item = HandoverMmapEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.mmap.size {
            self.count += 1;
            Some(self.mmap.entries[self.count - 1])
        }
        else {
            None
        }
    }
}

/// The Handover's framebuffer structure.
#[repr(C)]
pub struct HandoverFramebuffer {
    /// If true, the framebuffer exists, false otherwise.
    pub present: bool,
    /// The address to the first byte of the framebuffer.
    pub address: *mut u8,

    /// Width of the framebuffer.
    pub width: u32,
    /// Height of the framebuffer.
    pub height: u32,

    /// Pitch of the framebuffer.
    pub pitch: u32,
    /// Bpp of the framebuffer.
    pub bpp: u32,
}

impl HandoverFramebuffer {
    /// Returns a null framebuffer, **not present**.
    pub const fn null() -> Self {
        Self {
            present: false,
            address: core::ptr::null_mut(),
            width: 0,
            height: 0,
            pitch: 0,
            bpp: 0,
        }
    }
}

/// Structure of a Handover module.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HandoverModule {
    /// Size of the module.
    pub size: usize,
    /// Address of the module.
    pub address: usize,
    /// Name of the module.
    pub module_name: [u8; 32],
}

impl HandoverModule {
    /// Returns a null Handover module.
    pub const fn null() -> Self {
        Self {
            size: 0,
            address: 0,
            module_name: [0 as u8; 32],
        }
    }
}

/// Max number of Handover modules.
pub const MAX_MODULE_COUNT: usize = 16;

/// Structure containing all Handover modules.
#[repr(C)]
pub struct HandoverModules {
    /// Count of modules.
    pub size: usize,
    /// Modules array.
    pub modules: [HandoverModule; MAX_MODULE_COUNT],
}

impl HandoverModules {
    /// Returns an empty list of modules.
    pub const fn empty() -> HandoverModules {
        HandoverModules {
            size: 0,
            modules: [HandoverModule::null(); MAX_MODULE_COUNT],
        }
    }
}

/// The Handover's command line.
#[repr(C)]
pub struct HandoverCmdLines {
    pub present: bool,
    pub cmd_line: [u8; 32],
}

impl HandoverCmdLines {
    pub const fn null() -> Self {
        Self {
            present: false,
            cmd_line: [0 as u8; 32],
        }
    }
}

/// Handover's main structure, containing everything the kernel can need.
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

impl Handover {
    pub const fn null() -> Self {
        Self {
            tag: HANDOVER_TAG,

            mmap: HandoverMmap::empty(),
            framebuffer: HandoverFramebuffer::null(),
            modules: HandoverModules::empty(),

            rsdp: 0,
            fdt: 0,

            cmd_lines: HandoverCmdLines::null(),
        }
    }
}
