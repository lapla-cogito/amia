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

impl Virtq {
    pub fn new(idx: u32) -> *mut Self {
        let vitrq_addr = unsafe {
            crate::paging::alloc_pages(
                crate::util::align_up(
                    core::mem::size_of::<Virtq>() as u64,
                    crate::constants::PAGE_SIZE as u64,
                ) / crate::constants::PAGE_SIZE as u64,
            )
        };

        let virtq = unsafe { (vitrq_addr as u64 as *mut Virtq).as_mut().unwrap() };
        virtq.idx = idx;
        virtq.used_idx = unsafe {
            (&mut (virtq.used) as *const VirtqUsed as *const u8)
                .offset(core::mem::offset_of!(VirtqUsed, idx) as isize)
        } as *mut u16;

        virtq
    }

    pub fn reset(&mut self) {
        self.avail.idx = 0;
        self.used.idx = 0;
        self.used.ring = [VirtqUsedElem { id: 0, len: 0 }; crate::constants::VIRTIO_ENTRY];
        self.last_used_idx = 0;
    }

    pub fn is_full(&self) -> bool {
        self.last_used_idx != self.used_idx as u16
    }
}
