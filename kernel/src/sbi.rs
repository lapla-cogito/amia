pub struct Sbiret {
    pub error: i64,
    pub value: i64,
}

unsafe fn sbi_call(
    arg0: i64,
    arg1: i64,
    arg2: i64,
    arg3: i64,
    arg4: i64,
    arg5: i64,
    fid: i64,
    eid: i64,
) -> Sbiret {
    let mut error;
    let mut value;
    core::arch::asm!(
        "ecall",
        inout("a0") arg0 => error, inout("a1") arg1 => value,
        in("a2") arg2, in("a3") arg3, in("a4") arg4, in("a5") arg5,
        in("a6") fid, in("a7") eid
    );

    Sbiret { error, value }
}

#[no_mangle]
pub fn putchar(c: u8) {
    unsafe {
        sbi_call(c as i64, 0, 0, 0, 0, 0, 0, 1);
    }
}

#[no_mangle]
pub fn getchar() -> i64 {
    unsafe {
        let ret = sbi_call(0, 0, 0, 0, 0, 0, 0, 2);
        ret.error
    }
}
