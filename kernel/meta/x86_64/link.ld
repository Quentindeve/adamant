ENTRY(entry_point)

SECTIONS
{
    kernel_phys_offset = 0xffffffff80200000;
    . = kernel_phys_offset;
    
    .stivale2hdr ALIGN(4K):
    {
        KEEP(*(.stivale2hdr))
    }
    
    .text ALIGN(4K):
    {
        *(.text*)
    }
    
    .rodata ALIGN(4K):
    {
        *(.rodata*)
    }
    
    .data  ALIGN(4K):
    {
        *(.data*)
    }
    
    .bss  ALIGN(4K) :
    {
        *(COMMON)
        *(.bss*)
    }
}