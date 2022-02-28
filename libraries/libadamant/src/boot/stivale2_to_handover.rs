use stivale_boot::v2::{
    StivaleFramebufferTag, StivaleMemoryMapEntryType, StivaleMemoryMapTag, StivaleStruct,
};

use super::handover::*;

pub fn _stivale2_to_handover(stivale_struct: &StivaleStruct, handover: &mut Handover) {
    handover.tag = HANDOVER_TAG;

    let stivale2_mmap = stivale_struct.memory_map();
    match stivale2_mmap {
        Some(mmap) => stivale2_mmap_to_handover(mmap, handover),
        None => panic!("Stivale2 didn't provide any memory map."),
    }

    let stivale2_framebuffer = stivale_struct.framebuffer();
    stivale2_framebuffer_to_handover(stivale2_framebuffer, handover);

    stivale2_modules_to_handover(stivale_struct, handover);

    match stivale_struct.rsdp() {
        Some(rsdp) => handover.rsdp = rsdp.rsdp as usize,
        _ => handover.rsdp = 0,
    }

    handover.fdt = 0;
}

fn stivale2_mmap_to_handover(stivale2_mmap: &'static StivaleMemoryMapTag, handover: &mut Handover) {
    let size = stivale2_mmap.entries_len;

    let mut entries = [HandoverMmapEntry::null(); HANDOVER_MMAP_MAX_SIZE];
    let mut size = 0usize;

    for entry in stivale2_mmap.iter().enumerate() {
        let index = entry.0;

        // We can't know all we need will be stored in the memory map so we don't continue.
        if index == HANDOVER_MMAP_MAX_SIZE {
            panic!(
                "Stivale2 memory map ({}) is larger than Handover's one ({}).",
                size, HANDOVER_MMAP_MAX_SIZE
            );
        }

        size += 1;
        let entry = entry.1;

        // Determine which type the new entry will be
        let mmap_type = match entry.entry_type {
            StivaleMemoryMapEntryType::Usable => HandoverMmapType::MmapFree,

            StivaleMemoryMapEntryType::Reserved => HandoverMmapType::MmapReserved,
            StivaleMemoryMapEntryType::AcpiNvs => HandoverMmapType::MmapReserved,
            StivaleMemoryMapEntryType::BadMemory => HandoverMmapType::MmapReserved,

            StivaleMemoryMapEntryType::Kernel => HandoverMmapType::MmapUsed,
            StivaleMemoryMapEntryType::Framebuffer => HandoverMmapType::MmapUsed,

            StivaleMemoryMapEntryType::AcpiReclaimable => HandoverMmapType::MmapReclaimable,
            StivaleMemoryMapEntryType::BootloaderReclaimable => HandoverMmapType::MmapReclaimable,
        };

        entries[index] = HandoverMmapEntry {
            begin: entry.base as usize,
            end: (entry.base + entry.length) as usize,
            mmap_type: mmap_type,
        };
    }

    handover.mmap = HandoverMmap { size, entries }
}

fn stivale2_framebuffer_to_handover(
    stivale2_framebuffer: Option<&StivaleFramebufferTag>,
    handover: &mut Handover,
) {
    match stivale2_framebuffer {
        Some(framebuffer) => {
            handover.framebuffer = HandoverFramebuffer {
                present: true,
                address: framebuffer.framebuffer_addr as *mut u8,

                width: framebuffer.framebuffer_width as u32,
                height: framebuffer.framebuffer_height as u32,

                pitch: framebuffer.framebuffer_pitch as u32,
                bpp: framebuffer.framebuffer_bpp as u32,
            }
        }

        None => {
            handover.framebuffer = HandoverFramebuffer {
                present: false,
                address: core::ptr::null_mut(),
                width: 0,
                height: 0,
                pitch: 0,
                bpp: 0,
            }
        }
    }
}

fn stivale2_modules_to_handover(stivale2_struct: &StivaleStruct, handover: &mut Handover) {
    let modules = stivale2_struct.modules();

    match modules {
        Some(modules) => {
            let mut mod_count = 0usize;
            for module in modules.iter().enumerate() {
                mod_count += 1;
                if mod_count > MAX_MODULE_COUNT {
                    panic!(
                        "Stivale2 has too much modules ({}) for Handover ({})",
                        modules.module_len, MAX_MODULE_COUNT
                    );
                }

                let module_index = module.0;
                let module = module.1;

                let stivale2_mod_name_shrink = &module.string[0..32];
                let mut stivale2_mod_name = [0; 32];
                stivale2_mod_name.clone_from_slice(stivale2_mod_name_shrink);

                let handover_module_name = unsafe {
                    core::mem::transmute::<[u8; 32], [libc::c_char; 32]>(stivale2_mod_name)
                };

                handover.modules.modules[module_index] = HandoverModule {
                    address: module.start as usize,
                    size: (module.end - module.start) as usize,
                    module_name: handover_module_name,
                }
            }
            handover.modules.size = mod_count;
        }
        None => handover.modules = HandoverModules::empty(),
    }
}
