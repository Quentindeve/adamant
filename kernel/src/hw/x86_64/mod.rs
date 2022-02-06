pub mod com;
pub mod gdt;
pub mod port;

/// The x86_64 arch entry point.
extern "C" fn entry_point() -> ! {
    gdt::setup_gdt();
    loop {}
}
