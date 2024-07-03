#![no_std]
#![no_main]
#![feature(c_variadic)]
#![feature(fn_align)]
#![feature(naked_functions)]
#![feature(asm_const)]

mod constants;
mod paging;
mod process;
mod sbi;
mod types;
mod util;

use crate::util::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u64;
    static mut __free_ram: u8;
    static __free_ram_end: u8;
    static __kernel_base: u8;
}

#[no_mangle]
fn kernel_main() {
    let bin_shell = include_bytes!("../shell.bin");

    unsafe {
        memset(
            &__bss as *const u8 as *mut u8,
            0,
            (__bss_end - __bss) as usize,
        );

        crate::paging::NEXT_PADDR = core::ptr::addr_of!(__free_ram) as u64;
        write_csr!("stvec", kernel_entry as usize);

        crate::process::IDLE_PROC =
            crate::process::create_process(crate::constants::NULL as *const u64, 0);
        (*(crate::process::IDLE_PROC)).pid = -1;
        crate::process::CURRENT_PROC = crate::process::IDLE_PROC;

        crate::process::create_process(bin_shell.as_ptr() as *const u64, bin_shell.len());
        crate::process::yield_proc();
    }

    unreachable!();
}

#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn boot() -> ! {
    core::arch::asm!(
        "mv sp, {stack_top}
        j {kernel_main}",
        stack_top = in(reg) &__stack_top,
        kernel_main = sym kernel_main,
    );
    loop {}
}

#[allow(dead_code)]
#[no_mangle]
extern "C" fn handle_trap(_frame: TrapFrame) {
    let mut scause: u32;
    let mut stval: u32;
    let mut sepc: u32;

    unsafe {
        core::arch::asm!("csrr {}, scause", out(reg) scause);
        core::arch::asm!("csrr {}, stval", out(reg) stval);
        core::arch::asm!("csrr {}, sepc", out(reg) sepc);
    }
    panic!("scause: {:x}, stval: {:x}, sepc: {:x}", scause, stval, sepc);
}

#[no_mangle]
#[link_section = ".text.boot"]
#[repr(align(8))]
#[naked]
pub unsafe extern "C" fn kernel_entry() {
    core::arch::asm!(
        "
        csrw sscratch, sp
        addi sp, sp, -4 * 31
        sd ra,  4 * 0(sp)
        sd gp,  4 * 1(sp)
        sd tp,  4 * 2(sp)
        sd t0,  4 * 3(sp)
        sd t1,  4 * 4(sp)
        sd t2,  4 * 5(sp)
        sd t3,  4 * 6(sp)
        sd t4,  4 * 7(sp)
        sd t5,  4 * 8(sp)
        sd t6,  4 * 9(sp)
        sd a0,  4 * 10(sp)
        sd a1,  4 * 11(sp)
        sd a2,  4 * 12(sp)
        sd a3,  4 * 13(sp)
        sd a4,  4 * 14(sp)
        sd a5,  4 * 15(sp)
        sd a6,  4 * 16(sp)
        sd a7,  4 * 17(sp)
        sd s0,  4 * 18(sp)
        sd s1,  4 * 19(sp)
        sd s2,  4 * 20(sp)
        sd s3,  4 * 21(sp)
        sd s4,  4 * 22(sp)
        sd s5,  4 * 23(sp)
        sd s6,  4 * 24(sp)
        sd s7,  4 * 25(sp)
        sd s8,  4 * 26(sp)
        sd s9,  4 * 27(sp)
        sd s10, 4 * 28(sp)
        sd s11, 4 * 29(sp)
        csrr a0, sscratch
        sd a0, 4 * 30(sp)
        mv a0, sp
        call handle_trap
        ld ra,  4 * 0(sp)
        ld gp,  4 * 1(sp)
        ld tp,  4 * 2(sp)
        ld t0,  4 * 3(sp)
        ld t1,  4 * 4(sp)
        ld t2,  4 * 5(sp)
        ld t3,  4 * 6(sp)
        ld t4,  4 * 7(sp)
        ld t5,  4 * 8(sp)
        ld t6,  4 * 9(sp)
        ld a0,  4 * 10(sp)
        ld a1,  4 * 11(sp)
        ld a2,  4 * 12(sp)
        ld a3,  4 * 13(sp)
        ld a4,  4 * 14(sp)
        ld a5,  4 * 15(sp)
        ld a6,  4 * 16(sp)
        ld a7,  4 * 17(sp)
        ld s0,  4 * 18(sp)
        ld s1,  4 * 19(sp)
        ld s2,  4 * 20(sp)
        ld s3,  4 * 21(sp)
        ld s4,  4 * 22(sp)
        ld s5,  4 * 23(sp)
        ld s6,  4 * 24(sp)
        ld s7,  4 * 25(sp)
        ld s8,  4 * 26(sp)
        ld s9,  4 * 27(sp)
        ld s10, 4 * 28(sp)
        ld s11, 4 * 29(sp)
        ld sp,  4 * 30(sp)
        sret
        ",
        options(noreturn)
    );
}
