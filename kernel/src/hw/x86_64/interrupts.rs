use core::{arch::asm, intrinsics::black_box};

use super::com::ComPort;

#[repr(C, packed)]
struct Registers {
    ds: u64,
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
type HandlerFunc = fn(rsp: &Registers);

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
extern "C" fn interrupt_handler(rsp: &Registers) {
    let int_no = rsp.int_no;
    if int_no <= 31 {
        EXCEPTION_HANDLERS[int_no as usize](rsp);
    }
}

fn exception_div_by_zero(rsp: &Registers) {
    super::com::write_text(ComPort::COM1, "Exception 0 triggered !");
}

fn exception_debug(rsp: &Registers) {}

fn exception_nmi(rsp: &Registers) {}

fn exception_breakpoint(rsp: &Registers) {}

fn exception_overflow(rsp: &Registers) {}

fn exception_bound(rsp: &Registers) {}

fn exception_invalid_opcode(rsp: &Registers) {}

fn exception_device_not_available(rsp: &Registers) {}

fn double_fault(rsp: &Registers) {}

fn exception_coprocessor(rsp: &Registers) {}

fn exception_invalid_tss(rsp: &Registers) {}

fn exception_missing_segment(rsp: &Registers) {}

fn exception_invalid_stack(rsp: &Registers) {}

fn exception_gpf(rsp: &Registers) {}

fn page_fault(rsp: &Registers) {}

fn reserved15(rsp: &Registers) {}

fn exception_fpu(rsp: &Registers) {}

fn align_fault(rsp: &Registers) {}

fn exception_machine_verif_fault(rsp: &Registers) {}

fn exception_simd(rsp: &Registers) {}

fn exception_virt(rsp: &Registers) {}

fn reserved_21_31(rsp: &Registers) {}
