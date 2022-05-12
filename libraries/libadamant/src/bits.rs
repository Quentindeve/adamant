//! This module defines all the code to create and manipulate a bitmap.

/// Structure representing a bitmap of [base; len[.
#[repr(C)]
pub struct Bits {
    pub len: usize,
    pub base: *mut u8,
}

impl Bits {
    /// Creates a new bitmap.
    /// Be careful: this function will panic if base is null or len is 0.
    pub fn new(base: *mut u8, len: usize) -> Self {
        assert_ne!(base, core::ptr::null_mut());
        assert!(len > 0);

        Self { base, len }
    }

    /// Get the element at the specific index.
    pub fn get(&self, index: usize) -> *mut u8 {
        assert!(index <= self.len - 1);

        ((self.base as usize) + index) as *mut u8
    }

    pub fn get_state(&self, index: usize) -> bool {
        let byte_index = byte_index(index);
        let bit_index = bit_index(index);
        unsafe { (*(self.get(byte_index)) >> bit_index) % 2 == 1 }
    }

    pub fn set(&mut self, index: usize, state: bool) {
        let byte_index = byte_index(index);
        let bit_index = bit_index(index);

        if state {
            unsafe {
                *(self.get(byte_index)) |= 1 << bit_index;
            }
        } else {
            unsafe {
                *(self.get(byte_index)) &= !(1 << (bit_index));
            }
        }
    }

    /// Sets [start_index; end_index[ bits to ``state``
    pub fn set_range(&mut self, start_index: usize, end_index: usize, state: bool) {
        assert!(end_index <= self.len);

        for i in start_index..=end_index {
            self.set(i, state);
        }
    }

    pub fn fill(&mut self, state: bool) {
        unsafe {
            if state {
                core::ptr::write_bytes(self.base, 0xff, self.len);
            } else {
                core::ptr::write_bytes(self.base, 0, self.len);
            }
        }
    }
}

#[inline(always)]
pub fn byte_index(index: usize) -> usize {
    index / 8
}

#[inline(always)]
pub fn bit_index(index: usize) -> usize {
    index % 8
}
