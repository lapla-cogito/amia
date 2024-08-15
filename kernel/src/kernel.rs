#![no_std]
#![no_main]
#![feature(c_variadic)]
#![feature(fn_align)]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod constants;
mod elf;
mod error;
mod mutex;
mod net;
mod paging;
mod process;
mod sbi;
mod test;
mod types;
mod util;
mod virtio;
mod virtio_net;

use crate::util::*;

pub use crate::error::{Error, Result};

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
    #[cfg(test)]
    test_main();

    unsafe { crate::virtio::init() };
    crate::virtio_net::virtq_init();

    for i in crate::virtio::VIRTIO_MMIO_BASE_CANDIDATE {
        let vendor_id = unsafe { core::ptr::read_volatile((i + 0x008) as *const u32) };
        let mut mac_addr: [u8; 6] = [0; 6];
        if vendor_id != 0 {
            print!("mac addr: ");
            let mac_addr_start = i + 0x100;
            for j in 0..6 {
                mac_addr[j] =
                    unsafe { core::ptr::read_volatile((mac_addr_start + (j as u64)) as *const u8) };
                print!("{:02x}", mac_addr[j]);
                if j != 5 {
                    print!(":");
                }
            }
            println!("");

            let arp_request =
                crate::net::arp::ArpEther::request(mac_addr, [10, 0, 2, 15], [10, 0, 2, 2]);
        }
    }

    let bin_shell = crate::elf::ElfHeader::new(include_bytes!("../shell"));

    unsafe {
        memset(
            &__bss as *const u8 as *mut u8,
            0,
            (__bss_end - __bss) as usize,
        );

        crate::paging::NEXT_PADDR = core::ptr::addr_of!(__free_ram) as u64;
        write_csr!("stvec", kernel_entry as usize);

        crate::process::IDLE_PROC =
            crate::process::create_process(crate::constants::NULL as *const crate::elf::ElfHeader);
        (*(crate::process::IDLE_PROC)).pid = -1;
        crate::process::CURRENT_PROC = crate::process::IDLE_PROC;
        crate::process::create_process(bin_shell);
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
extern "C" fn handle_trap(frame: *mut TrapFrame) {
    let mut scause: u64;
    let mut stval: u64;
    let mut sepc: u64;

    unsafe {
        core::arch::asm!("csrr {}, scause", out(reg) scause);
        core::arch::asm!("csrr {}, stval", out(reg) stval);
        core::arch::asm!("csrr {}, sepc", out(reg) sepc);
    }
    if scause == crate::constants::SCAUSE_ECALL {
        handle_syscall(frame);
        sepc += 4;
    } else {
        panic!("scause: {:x}, stval: {:x}, sepc: {:x}", scause, stval, sepc);
    }

    write_csr!("sepc", sepc);
}

fn handle_syscall(f: *mut TrapFrame) {
    let f = unsafe { f.as_mut().unwrap() };

    match f.a3 {
        crate::constants::SYS_PUTCHAR => crate::sbi::putchar(f.a0 as u8),
        crate::constants::SYS_GETCHAR => loop {
            let c = crate::sbi::getchar();
            if c != -1 {
                f.a0 = c as u64;
                break;
            }
        },
        crate::constants::SYS_EXIT => {
            let current_proc = unsafe { crate::process::CURRENT_PROC.as_mut().unwrap() };
            println!("process {} exited", current_proc.pid);
            current_proc.state = crate::constants::PROC_EXITED;
            unsafe {
                crate::process::yield_proc();
            }
            unreachable!();
        }
        _ => panic!("unknown syscall: a3={:x}", f.a3 as u32),
    }
}

#[no_mangle]
#[link_section = ".text.boot"]
#[naked]
pub unsafe extern "C" fn kernel_entry() {
    core::arch::asm!(
        "
        csrw sscratch, sp
        addi sp, sp, -8 * 31
        sd ra,  8 * 0(sp)
        sd gp,  8 * 1(sp)
        sd tp,  8 * 2(sp)
        sd t0,  8 * 3(sp)
        sd t1,  8 * 4(sp)
        sd t2,  8 * 5(sp)
        sd t3,  8 * 6(sp)
        sd t4,  8 * 7(sp)
        sd t5,  8 * 8(sp)
        sd t6,  8 * 9(sp)
        sd a0,  8 * 10(sp)
        sd a1,  8 * 11(sp)
        sd a2,  8 * 12(sp)
        sd a3,  8 * 13(sp)
        sd a4,  8 * 14(sp)
        sd a5,  8 * 15(sp)
        sd a6,  8 * 16(sp)
        sd a7,  8 * 17(sp)
        sd s0,  8 * 18(sp)
        sd s1,  8 * 19(sp)
        sd s2,  8 * 20(sp)
        sd s3,  8 * 21(sp)
        sd s4,  8 * 22(sp)
        sd s5,  8 * 23(sp)
        sd s6,  8 * 24(sp)
        sd s7,  8 * 25(sp)
        sd s8,  8 * 26(sp)
        sd s9,  8 * 27(sp)
        sd s10, 8 * 28(sp)
        sd s11, 8 * 29(sp)
        csrr a0, sscratch
        sd a0, 8 * 30(sp)
        mv a0, sp
        call handle_trap
        ld ra,  8 * 0(sp)
        ld gp,  8 * 1(sp)
        ld tp,  8 * 2(sp)
        ld t0,  8 * 3(sp)
        ld t1,  8 * 4(sp)
        ld t2,  8 * 5(sp)
        ld t3,  8 * 6(sp)
        ld t4,  8 * 7(sp)
        ld t5,  8 * 8(sp)
        ld t6,  8 * 9(sp)
        ld a0,  8 * 10(sp)
        ld a1,  8 * 11(sp)
        ld a2,  8 * 12(sp)
        ld a3,  8 * 13(sp)
        ld a4,  8 * 14(sp)
        ld a5,  8 * 15(sp)
        ld a6,  8 * 16(sp)
        ld a7,  8 * 17(sp)
        ld s0,  8 * 18(sp)
        ld s1,  8 * 19(sp)
        ld s2,  8 * 20(sp)
        ld s3,  8 * 21(sp)
        ld s4,  8 * 22(sp)
        ld s5,  8 * 23(sp)
        ld s6,  8 * 24(sp)
        ld s7,  8 * 25(sp)
        ld s8,  8 * 26(sp)
        ld s9,  8 * 27(sp)
        ld s10, 8 * 28(sp)
        ld s11, 8 * 29(sp)
        ld sp,  8 * 30(sp)
        sret
        ",
        options(noreturn)
    );
}
