//! This module defines all the code to create and manipulate a bitmap.

/// Structure representing a bitmap of [base; len[.
pub struct Bits {
    base: *mut u8,
    len: usize,
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
}
