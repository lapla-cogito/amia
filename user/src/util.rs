pub unsafe fn syscall(sysno: i64, mut args: [i64; 3]) -> i64 {
    let res: i64;

    core::arch::asm!(
        "ecall",
        in("a0") args[0],
        in("a1") args[1],
        in("a2") args[2],
        in("a3") sysno,
        lateout("a0") res,
    );

    res
}

pub fn putchar(c: char) -> i64 {
    unsafe { syscall(crate::constants::SYSCALL_PUTCHAR, [c as i64, 0, 0]) }
}

pub fn getchar() -> i64 {
    unsafe { syscall(crate::constants::SYSCALL_GETCHAR, [0, 0, 0]) }
}

pub fn print(s: &str) {
    for c in s.chars() {
        putchar(c);
    }
}

pub fn println(s: &str) {
    print(s);
    print("\n");
}
