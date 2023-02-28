/// https://www.nongnu.org/ext2-doc/ext2.html#def-superblock
#[repr(packed, C)]
pub struct Superblock {
    // Base
    pub inodes_count: u32,
    pub blocks_count: u32,
    pub r_blocks_count: u32,
    pub free_blocks_count: u32,
    pub free_inodes_count: u32,
    pub first_data_block: u32,
    pub log_block_size: u32,
    pub log_frag_size: u32,
    pub blocks_per_group: u32,
    pub frags_per_group: u32,
    pub inodes_per_group: u32,
    pub mtime: u32,
    pub wtime: u32,
    pub mnt_count: u16,
    pub max_mnt_count: u16,
    pub magic: u16,
    pub state: u16,
    pub errors: u16,
    pub minor_rev_level: u16,
    pub lastcheck: u32,
    pub checkinterval: u32,
    pub creator_os: u32,
    pub rev_level: u32,
    pub def_resuid: u16,
    pub def_resgid: u16,

    // EXT2_DYNAMIC_REV Specific
    pub first_ino: u32,
    pub inode_size: u16,
    pub block_group_nr: u16,
    pub feature_compat: u32,
    pub feature_incompat: u32,
    pub feature_ro_compat: u32,
    pub uuid: u128,
    pub volume_name: u128,
    pub last_mounted: [u128; 4],
    pub algo_bitmap: u32,

    // Performance Hints
    pub prealloc_blocks: u8,
    pub prealloc_dir_blocks: u8,
    pub __align1: u16,

    // Journaling Support
    pub journal_uuid: u128,
    pub journal_inum: u32,
    pub journal_dev: u32,
    pub last_orphan: u32,

    // Directory Indexing Support
    pub hash_seed: [u32; 4],
    pub def_hash_version: u8,
    pub __align2: [u8; 3],

    // Other Options
    pub default_mount_options: u32,
    pub first_meta_bg: u32,
    pub reserved: [u8; 760],
}

/// Size of the Superblock structure.
pub const SUPERBLOCK_SIZE: usize = core::mem::size_of::<Superblock>();

impl Superblock {
    /// Returns a Superblock from a slice of bytes, copying the buffer.
    pub const fn from_buffer_copy(buffer: &[u8; SUPERBLOCK_SIZE]) -> Self {
        unsafe { core::mem::transmute_copy(buffer) }
    }
}
