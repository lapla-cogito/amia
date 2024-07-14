#[repr(C, packed)]
#[derive(Clone, Copy)]
struct VirtqDesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct VirtqAvail {
    flags: u16,
    idx: u16,
    ring: [u16; 0],
    used_event: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct VirtqUsedElem {
    id: u32,
    len: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct VirtqUsed {
    flags: u16,
    idx: u16,
    ring: [VirtqUsedElem; crate::constants::VIRTIO_ENTRY],
    avail_event: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Virtq {
    desc: [VirtqDesc; crate::constants::VIRTIO_ENTRY],
    avail: VirtqAvail,
    used: VirtqUsed,
    _padding: [u8; (crate::constants::PAGE_SIZE
        - (crate::constants::SIZE % crate::constants::PAGE_SIZE))
        / core::mem::size_of::<u8>()],
    idx: u32,
    used_idx: *mut u16,
    last_used_idx: u16,
}
