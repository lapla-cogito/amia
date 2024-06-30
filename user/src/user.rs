#![no_std]
#![no_main]
#![feature(naked_functions)]

mod shell;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // println!("{info}");
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

extern "C" {
    static __stack_top: u64;
}

#[link_section = ".text.start"]
#[naked]
#[no_mangle]
unsafe extern "C" fn start() {
    core::arch::asm!(
        "
        la sp, {stack_top}
        call main
        ",
        stack_top = sym __stack_top,
        options(noreturn)
    );
}
