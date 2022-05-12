use std::alloc::Layout;

use libadamant::bits::{bit_index, byte_index, Bits};

fn alloc_bitmap() -> Bits {
    let base_ptr = unsafe { std::alloc::alloc(Layout::new::<[u8; 32]>()) };
    let mut bits = Bits::new(base_ptr, 32);
    bits.fill(false);
    bits
}

#[test]
fn bitmap_get_bit_works() {
    let bit_index = 5usize;
    let mut bitmap = alloc_bitmap();
    bitmap.set(bit_index, true);

    assert_eq!(bitmap.get_state(bit_index), true);
}

#[test]
fn bitmap_set_range_works() {
    let mut bitmap = alloc_bitmap();

    bitmap.set_range(0, 30, true);

    for i in 0..3 {
        let byte = unsafe { *bitmap.get(i) };
        assert_eq!(byte, 0xff);
    }

    let problematic_byte = unsafe { *bitmap.get(3) };
    assert_eq!(problematic_byte, 0x7F);
}

#[test]
fn byte_index_works() {
    assert_eq!(byte_index(8), 1);
}

#[test]
fn bit_index_works() {
    assert_eq!(bit_index(33), 1);
}
