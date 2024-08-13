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
pub struct VirtioChainEntry {
    pub idx: u16,
    pub addr: crate::types::PaddrT,
    pub len: u32,
    pub writeable: bool,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Virtq {
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

    pub fn send(&mut self, chain: &[VirtioChainEntry]) {
        let mut desc_idx = 0;
        let avail_idx = self.avail.idx as usize;

        for i in 0..chain.len() {
            let entry = &chain[i];
            let desc = &mut self.desc[desc_idx];
            desc.addr = entry.addr;
            desc.len = entry.len;
            desc.flags = if entry.writeable { 2 } else { 1 };
            desc.next = (desc_idx + 1) as u16;

            desc_idx += 1;
        }

        self.desc[desc_idx - 1].next = 0;

        self.avail.ring[avail_idx] = 0;
        self.avail.idx += 1;
    }
}

pub struct Dmabuf {
    pub paddr: crate::types::PaddrT,
    pub vaddr: crate::types::VaddrT,
    pub entry_size: usize,
    pub num_entries: usize,
    pub used: [bool; crate::constants::VIRTIO_ENTRY],
}

impl Dmabuf {
    pub fn alloc_dmabuf(&mut self, paddr: &mut crate::types::PaddrT) -> Result<(), u32> {
        for i in 0..self.num_entries {
            if !self.used[i] {
                self.used[i] = true;
                let offset = i * self.entry_size;
                let tmp = self.paddr + offset as u64;
                *paddr = tmp;
                return Ok(());
            }
        }
        Err(crate::constants::VIRTIO_ERR_NO_BUF)
    }

    pub fn free_dmabuf(&mut self, paddr: crate::types::PaddrT) {
        let offset = (paddr - self.paddr) as usize;
        let i = offset / self.entry_size;
        self.used[i] = false;
    }

    pub fn p2v(&self, paddr: crate::types::PaddrT) -> Result<crate::types::VaddrT, u32> {
        if paddr < self.paddr
            || paddr >= self.paddr + self.entry_size as u64 * self.num_entries as u64
        {
            return Err(crate::constants::VIRTIO_ERR_OUT_OF_INDEX);
        }

        Ok(self.vaddr + (paddr - self.paddr))
    }
}

pub struct VirtioMmio {
    pub base: crate::types::VaddrT,
    pub num_queues: u32,
    pub queues: [*mut Virtq],
}

impl VirtioMmio {
    pub fn notify(&self, idx: u32) {
        unsafe {
            core::ptr::write_volatile((self.base + 0x50 + 4 * idx as u64) as *mut u32, 0);
        }
    }
}

#[test_case]
fn test_virtq() {
    let virtq = Virtq::new(0);
    assert!(!virtq.is_null());
}
