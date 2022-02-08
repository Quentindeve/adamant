fn main() {
    make_x86_64_specific();
}

fn make_x86_64_specific() {
    // Compiles x86_64 assembly
    nasm_rs::Build::new()
        .file("src/hw/x86_64/gdt.s")
        .file("src/hw/x86_64/helpers.s")
        .file("src/hw/x86_64/interrupts.s")
        .flag("-f elf64")
        .flag("-F dwarf")
        .compile("adamant-x86_64")
        .unwrap();
}
