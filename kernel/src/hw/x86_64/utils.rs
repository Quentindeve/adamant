#[macro_export]
macro_rules! dump_rip {
    () => {
        use libadamant::{print, println};
        let mut rip: u64;
        asm!("lea rax, [rip]", out("rax") rip);
        println!("RIP: {:x}", rip);
    };
}
