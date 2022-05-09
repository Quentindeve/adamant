pub mod range;

pub const PAGE_SIZE: usize = 4 * 1024; // 4 KiB

#[inline(always)]
pub const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[inline(always)]
pub const fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}