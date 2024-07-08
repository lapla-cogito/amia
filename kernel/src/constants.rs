pub const NULL: *const core::ffi::c_void = core::ptr::null();

pub const SBI_SUCCESS: isize = 0;
pub const SBI_ERR_FAILED: isize = -1;
pub const SBI_ERR_NOT_SUPPORTED: isize = -2;
pub const SBI_ERR_INVALID_PARAM: isize = -3;
pub const SBI_ERR_DENIED: isize = -4;
pub const SBI_ERR_INVALID_ADDRESS: isize = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: isize = -6;
pub const SBI_ERR_ALREADY_STARTED: isize = -7;
pub const SBI_ERR_ALREADY_STOPPED: isize = -8;

pub const PROCS_MAX: usize = 8;
pub const PROC_UNUSED: i64 = 0;
pub const PROC_READY: i64 = 1;
pub const PROC_EXITED: i64 = 2;

pub const PAGE_SIZE: u64 = 4096;

pub const SATP_SV39: u64 = 8 << 60;
pub const PAGE_V: u64 = 1 << 0;
pub const PAGE_R: u64 = 1 << 1;
pub const PAGE_W: u64 = 1 << 2;
pub const PAGE_X: u64 = 1 << 3;
pub const PAGE_U: u64 = 1 << 4;

pub const USER_BASE: u64 = 0x1000000;
pub const SSTATUS_SPIE: u64 = 1 << 5;
pub const SSTATUS_SUM: u64 = 1 << 18;

pub const SCAUSE_ECALL: u64 = 8;
pub const SYS_PUTCHAR: u64 = 1;
pub const SYS_GETCHAR: u64 = 2;
pub const SYS_EXIT: u64 = 3;
