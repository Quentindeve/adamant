/// 16bit value identifying the file system as Ext2.
/// /// https://www.nongnu.org/ext2-doc/ext2.html#s-magic
pub const EXT2_SUPER_MAGIC: u16 = 0xEF53;

/// s_state value marking a cleanly unmounted filesystem.
/// https://www.nongnu.org/ext2-doc/ext2.html#s-state
pub const EXT2_VALID_FS: u16 = 1;
/// s_state value marking a dirty unmounted filesystem. The OS should check this FS.
/// https://www.nongnu.org/ext2-doc/ext2.html#s-state
pub const EXT2_ERROR_FS: u16 = 2;

/// s_errors value indicating that the OS should continue as if nothing happened
/// https://www.nongnu.org/ext2-doc/ext2.html#s-errors
pub const EXT2_ERRORS_CONTINUE: u16 = 1;
/// s_errors value indicating that this FS should not be mounted another way than read-only
/// https://www.nongnu.org/ext2-doc/ext2.html#s-errors
pub const EXT2_ERRORS_RO: u16 = 2;
/// s_errors value that **must** cause a kernel panic
/// https://www.nongnu.org/ext2-doc/ext2.html#s-errors
pub const EXT2_ERRORS_PANIC: u16 = 3;

/// rev_level value indicating the Revision 0 of the Ext2 specification
/// https://www.nongnu.org/ext2-doc/ext2.html#s-rev-level
pub const EXT2_GOOD_OLD_REV: u32 = 0;

/// rev_level value indicating the Revision 1 of the Ext2 specification with variable inode sizes, extended attributes, etc.
/// Found https://www.nongnu.org/ext2-doc/ext2.html#s-rev-level
pub const EXT2_DYNAMIC_REV: u32 = 0;
