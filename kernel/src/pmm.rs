use core::mem::MaybeUninit;

use libadamant::{
    bits::Bits,
    boot::handover::HandoverMmap,
    mem::{range::*, *},
    print, println,
};

static mut BITMAP: MaybeUninit<Bits> = MaybeUninit::uninit();

pub unsafe fn pmm_initialize(mmap: &HandoverMmap) {
    // How much memory we have ?
    let mem_range = PmmRange::from_mmap(mmap);

    // Find a place to allocate the bitmap
    let required_space = mem_range.size() / PAGE_SIZE / 8;
    let mut is_allocated = false;
    for entry in mmap.entries {
        // We found a place to put the bitmap !
        if entry.length() > required_space {
            BITMAP = MaybeUninit::new(Bits::new(
                align_up(entry.begin, PAGE_SIZE) as *mut u8,
                required_space,
            ));
            println!("Allocated ! {:x}", entry.begin);
            is_allocated = true;
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
