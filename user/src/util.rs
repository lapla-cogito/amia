unsafe fn syscall(sysno: u64, arg0: u64, arg1: u64, arg2: u64) -> u64 {
    let mut result: u64;

    core::arch::asm!(
        "ecall",
        in("a0") arg0,
        in("a1") arg1,
        in("a2") arg2,
        in("a3") sysno,
        lateout("a0") result,
    );

    result
}

pub fn putchar(ch: u8) {
    unsafe {
        syscall(crate::constants::SYSCALL_PUTCHAR, ch as u64, 0, 0);
    }
}
