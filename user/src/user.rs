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
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

extern "C" {
    static __stack_top: u32;
}

#[link_section = ".text.start"]
#[no_mangle]
pub unsafe extern "C" fn start() {
    core::arch::asm!(
        "mv sp, {stack_top}",
        "j {main}",
        "j {exit}",
        stack_top = in(reg) &__stack_top,
        main = sym crate::shell::main,
        exit = sym exit,
        options(noreturn)
    );
}
