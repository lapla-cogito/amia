#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(fn_align)]

mod constants;
mod shell;
mod util;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // println!("{info}");
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

#[no_mangle]
fn exit() {
    loop {}
}

extern "C" {
    static __stack_top: u32;
}

#[link_section = ".text.start"]
#[naked]
#[repr(align(8))]
#[no_mangle]
unsafe extern "C" fn start() {
    core::arch::asm!(
        "
        ld sp, {stack_top}
        call main
        call exit
        ",
        stack_top = sym __stack_top,
        options(noreturn)
    );
}
