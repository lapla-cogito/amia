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

pub const PAGE_SIZE: usize = 4096;

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

// https://docs.oasis-open.org/virtio/virtio/v1.2/csd01/virtio-v1.2-csd01.html#x1-110001
pub const VIRTIO_STATUS_ACK: u64 = 1;
pub const VIRTIO_STATUS_DRIVER: u64 = 2;
pub const VIRTIO_STATUS_FAILED: u64 = 128;
pub const VIRTIO_STATUS_FEATURE_OK: u64 = 8;
pub const VIRTIO_STATUS_DRIVER_OK: u64 = 4;
pub const VIRTIO_STATUS_NEEDS_RESET: u64 = 64;

// https://docs.oasis-open.org/virtio/virtio/v1.2/csd01/virtio-v1.2-csd01.html#x1-430005
pub const VIRTQ_DESC_F_NEXT: u64 = 1;
pub const VIRTQ_DESC_F_WRITE: u64 = 2;
pub const VIRTQ_DESC_F_INTERRUPT: u64 = 4;

pub const VIRTIO_ENTRY: usize = 16;
pub const VIRTIO_NET_MAX_PACKET_SIZE: usize = 1500;
pub const VIRTIO_NET_BASE: usize = 0x10002000;

pub const VIRTIO_NET_RX_QUEUE_IDX: u32 = 0;
pub const VIRTIO_NET_TX_QUEUE_IDX: u32 = 1;

const SIZE_OF_U16: usize = core::mem::size_of::<u16>();
const SIZE_OF_U32: usize = core::mem::size_of::<u32>();
const SIZE_OF_U64: usize = core::mem::size_of::<u64>();

pub const SIZE: usize = (SIZE_OF_U64 + SIZE_OF_U16 + SIZE_OF_U32 + SIZE_OF_U16) * VIRTIO_ENTRY
    + (SIZE_OF_U16 + SIZE_OF_U16 + SIZE_OF_U16 * VIRTIO_ENTRY);

pub const VIRTIO_ERR_TOO_LARGE: u32 = 1;
pub const VIRTIO_ERR_TRY_AGAIN: u32 = 2;
pub const VIRTIO_ERR_NO_BUF: u32 = 3;
pub const VIRTIO_ERR_OUT_OF_INDEX: u32 = 4;
