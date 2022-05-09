use core::{arch::asm, mem::MaybeUninit};

use libadamant::{
    bits::Bits,
    boot::handover::{HandoverMmap, HandoverMmapType},
    mem::{range::*, *},
    print, println,
};

static mut BITMAP: MaybeUninit<Bits> = MaybeUninit::uninit();

pub unsafe fn pmm_initialize(mmap: &HandoverMmap) {
    // How much memory we have ?
    let mem_range = PmmRange::from_mmap(mmap);
    println!("Mem size: {} kib", mem_range.size() / 1024);
    // Find a place to allocate the bitmap
    let required_space = (mem_range.size() / PAGE_SIZE) / 8;
    println!("bitmap must be {} kib", required_space / 1024);
    let mut is_allocated = false;

    for entry in mmap.iter() {
        // We found a place to put the bitmap !
        if (entry.mmap_type == HandoverMmapType::MmapFree) && entry.length() > required_space {
            BITMAP = MaybeUninit::new(Bits::new(
                align_up(entry.begin, PAGE_SIZE) as *mut u8,
                required_space,
            ));
            is_allocated = true;
            break;
        }
    }

    // No PMM no kernel !
    if !is_allocated {
        panic!("Couldn't allocate the pmm's bitmap.");
    }

    println!(
        "Bitmap allocated: base = {:x}, len = {}",
        BITMAP.assume_init_ref().base as usize,
        BITMAP.assume_init_ref().len
    );
}
