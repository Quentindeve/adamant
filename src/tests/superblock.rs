use crate::{
    constants::{EXT2_DYNAMIC_REV, EXT2_GOOD_OLD_REV, EXT2_SUPER_MAGIC},
    structures::superblock::{Superblock, SUPERBLOCK_SIZE},
};

#[test]
fn superblock_size_is_1024_bytes() {
    assert_eq!(crate::structures::superblock::SUPERBLOCK_SIZE, 1024);
}

#[test]
fn test_parse_superblock() {
    let slice: &[u8; 1024] = &super::file::TEST_FS[1024..2048].try_into().unwrap();
    let superblock = Superblock::from_buffer_copy(slice);

    let magic = superblock.magic;
    assert_eq!(magic, EXT2_SUPER_MAGIC, "Bad Superblock Magic");

    // Asserts that block size is positive according to the specification
    let block_size = 1024 << superblock.log_block_size;
    assert!(block_size > 0, "Bad block size: {}", block_size);

    let rev = superblock.rev_level;
    assert!(
        rev == EXT2_DYNAMIC_REV || rev == EXT2_GOOD_OLD_REV,
        "Bad/Unsupported Revision Number"
    );
}
