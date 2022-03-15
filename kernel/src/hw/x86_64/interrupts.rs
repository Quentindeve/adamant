use core::{arch::asm, intrinsics::black_box};

use libadamant::{print,println};

use super::com::ComPort;

#[repr(C, packed)]
#[derive(Debug)]
struct Registers {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,

    rbp: u64,
    rdi: u64,
    rsi: u64,
    rdx: u64,
    rcx: u64,
    rbx: u64,
    rax: u64,

    int_no: u64,
    error_code: u64,

    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

// TODO: Maybe got optimized don't rage kiddo :^)
type HandlerFunc = fn(_rsp: &Registers);

#[used]
static EXCEPTION_HANDLERS: [HandlerFunc; 32] = black_box([
    exception_div_by_zero,
    exception_debug,
    exception_nmi,
    exception_breakpoint,
    exception_overflow,
    exception_bound,
    exception_invalid_opcode,
    exception_device_not_available,
    double_fault,
    exception_coprocessor,
    exception_invalid_tss,
    exception_missing_segment,
    exception_invalid_stack,
    exception_gpf,
    page_fault,
    reserved15,
    exception_fpu,
    align_fault,
    exception_machine_verif_fault,
    exception_simd,
    exception_virt,
    reserved_21_31,
    reserved_21_31,
    reserved_21_31,
    reserved_21_31,
    reserved_21_31,
    reserved_21_31,
    reserved_21_31,
    reserved_21_31,
    reserved_21_31,
    reserved_21_31,
    reserved_21_31,
]);

pub fn disable_interrupts() {
    unsafe {
        asm!("cli");
    }
}

pub fn enable_interrupts() {
    unsafe {
        asm!("sti");
    }
}

#[no_mangle]
extern "C" fn interrupt_handler(rsp: &Registers) -> &Registers {
    let int_no = rsp.int_no;
    let rip = rsp.rip;
    let code = rsp.error_code;
    println!("Rip = {:x}, int_no = {}, error_code = {:b}", rip, int_no, code);

    if int_no <= 31 {
        EXCEPTION_HANDLERS[int_no as usize](rsp);
    }

    return rsp;
}

fn exception_div_by_zero(_rsp: &Registers) {
    println!("Test ???");
}

fn exception_debug(_rsp: &Registers) {}

fn exception_nmi(_rsp: &Registers) {}

fn exception_breakpoint(_rsp: &Registers) {}

fn exception_overflow(_rsp: &Registers) {}

fn exception_bound(_rsp: &Registers) {}

fn exception_invalid_opcode(_rsp: &Registers) {}

fn exception_device_not_available(_rsp: &Registers) {}

fn double_fault(_rsp: &Registers) {}

fn exception_coprocessor(_rsp: &Registers) {}

fn exception_invalid_tss(_rsp: &Registers) {}

fn exception_missing_segment(_rsp: &Registers) {}

fn exception_invalid_stack(_rsp: &Registers) {}

fn exception_gpf(_rsp: &Registers) {}

fn page_fault(_rsp: &Registers) {}

fn reserved15(_rsp: &Registers) {}

fn exception_fpu(_rsp: &Registers) {}

fn align_fault(_rsp: &Registers) {}

fn exception_machine_verif_fault(_rsp: &Registers) {}

fn exception_simd(_rsp: &Registers) {}

fn exception_virt(_rsp: &Registers) {}

fn reserved_21_31(_rsp: &Registers) {}
