use crate::boot::handover::HandoverMmap;

/// Half-opened range, [start; end[
pub struct PmmRange {
    pub start: usize,
    pub end: usize,
}

impl PmmRange {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub const fn from_mmap(mmap: &HandoverMmap) -> Self {
        let mmap_start = mmap.entries[0].begin;
        let mmap_end = mmap.entries[mmap.size - 1].end;

        Self {
            start: mmap_start,
            end: mmap_end,
        }
    }

    pub const fn size(&self) -> usize {
        self.end - self.start - 1
    }
}
