use core::panic::PanicInfo;

#[panic_handler]
fn panic(_infos: &PanicInfo) -> ! {
    loop {}
}
