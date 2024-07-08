unsafe fn sbi_call(
    mut arg0: i64,
    mut arg1: i64,
    arg2: i64,
    arg3: i64,
    arg4: i64,
    arg5: i64,
    fid: i64,
    eid: i64,
) -> Result<i64, crate::types::SbiErr> {
    core::arch::asm!(
        "ecall",
        inout("a0") arg0 => arg0,
        inout("a1") arg1 => arg1,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
        in("a5") arg5,
        in("a6") fid,
        in("a7") eid,
    );

    let err = arg0 as isize;
    if err == crate::constants::SBI_SUCCESS {
        Ok(arg1)
    } else {
        Err(err as crate::types::SbiErr)
    }
}

pub fn putchar(c: char) -> Result<(), crate::types::SbiErr> {
    unsafe {
        let _res = sbi_call(c as i64, 0, 0, 0, 0, 0, 1, 1)?;
    }
    Ok(())
}

pub fn memset(dest: *mut u8, val: u8, count: usize) {
    for i in 0..count {
        unsafe {
            *dest.add(i) = val;
        }
    }
}

pub fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: crate::types::SizeT) {
    unsafe {
        let mut p_dst = dst as *mut u8;
        let mut p_src = src as *const u8;

        for _ in 0..n {
            *p_dst = *p_src;
            p_dst = p_dst.add(1);
            p_src = p_src.add(1);
        }
    }
}

pub fn strcpy(dst: *mut i8, src: *const i8) -> *mut i8 {
    unsafe {
        let mut p_dst = dst;
        let mut p_src = src;

        while *p_src != 0 {
            *p_dst = *p_src;
            p_dst = p_dst.add(1);
            p_src = p_src.add(1);
        }

        *p_dst = 0;

        dst
    }
}

pub fn strcmp(s1: *const u8, s2: *const u8) -> i32 {
    unsafe {
        let mut p_s1 = s1;
        let mut p_s2 = s2;

        while *p_s1 != 0 && *p_s1 == *p_s2 {
            p_s1 = p_s1.add(1);
            p_s2 = p_s2.add(1);
        }

        (*p_s1).cmp(&(*p_s2)) as i32
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::Writer, $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        use $crate::print;
        print!("{}\n", format_args!($($arg)*));
    });
}

pub struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            unsafe {
                core::arch::asm!(
                    "ecall",
                    in("a0") c,
                    in("a6") 0,
                    in("a7") 1,
                );
            }
        }
        Ok(())
    }
}

#[repr(C)]
#[repr(packed)]
pub struct TrapFrame {
    pub ra: u64,
    pub gp: u64,
    pub tp: u64,
    pub t0: u64,
    pub t1: u64,
    pub t2: u64,
    pub t3: u64,
    pub t4: u64,
    pub t5: u64,
    pub t6: u64,
    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
    pub a3: u64,
    pub a4: u64,
    pub a5: u64,
    pub a6: u64,
    pub a7: u64,
    pub s0: u64,
    pub s1: u64,
    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub s8: u64,
    pub s9: u64,
    pub s10: u64,
    pub s11: u64,
    pub sp: u64,
}

#[macro_export]
macro_rules! read_csr {
    ($csr:literal) => {{
        let mut val: u64;
        unsafe {
            ::core::arch::asm!(concat!("csrr {}, ", $csr), out(reg) val);
        }
        val
    }};
}

#[macro_export]
macro_rules! write_csr {
    ($csr:literal, $val:expr) => {{
        let val = $val;
        unsafe {
            ::core::arch::asm!(concat!("csrw ", $csr, ", {}"), in(reg) val);
        }
    }};
}

pub const fn align_up(value: u64, align: u64) -> u64 {
    if value % align == 0 {
        value
    } else {
        value + align - (value % align)
    }
}
