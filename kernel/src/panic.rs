use core::panic::PanicInfo;
use libadamant::{print, println};

#[panic_handler]
fn panic(infos: &PanicInfo) -> ! {
    if let Some(location) = infos.location() && let Some(message) = infos.message() {
        println!("[PANIC] Panic at {}: {}", location, message);        
    }
    loop {}

}
