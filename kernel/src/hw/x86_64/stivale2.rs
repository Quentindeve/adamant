use core::hint::black_box;

use stivale_boot::v2::{StivaleFramebufferHeaderTag, StivaleHeader};

const STACK_LENGTH: usize = 16 * 1024; // 16 KiB of stack.

static STACK: [u8; STACK_LENGTH] = [0; STACK_LENGTH];

const STIVALE_FRAMEBUFFER: StivaleFramebufferHeaderTag =
    StivaleFramebufferHeaderTag::new().framebuffer_bpp(24);

#[used]
#[link_section = ".stivale2hdr"]
static STIVALE_HEADER: StivaleHeader = StivaleHeader::new()
    .stack(&STACK[STACK_LENGTH - 1] as *const u8)
    .tags((&STIVALE_FRAMEBUFFER as *const StivaleFramebufferHeaderTag).cast());
