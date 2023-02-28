pub const TEST_FS: &[u8] = include_bytes!("../../sample_disk.img");

pub const fn block(index: usize) -> &'static [u8; 1024] {
    TEST_FS[1024 * index..1024.index + 1024].try_into().unwrap()
}
