use crate::{print, println};

pub const VIRTIO_MMIO_BASE_CANDIDATE: [u64; 8] = [
    0x10001000, 0x10002000, 0x10003000, 0x10004000, 0x10005000, 0x10006000, 0x10007000, 0x10008000,
];

const TCP_SYNACK_PACKET: [u8; 57] = [
    0x00, 0x0c, 0x29, 0x3f, 0x3f, 0x3f, 0x00, 0x0c, 0x29, 0x3f, 0x3f, 0x3f, 0x08, 0x00, 0x45, 0x00,
    0x00, 0x28, 0x00, 0x00, 0x40, 0x00, 0x40, 0x06, 0x00, 0x00, 0x0a, 0x00, 0x02, 0x15, 0x0a, 0x00,
    0x02, 0x14, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x02, 0x20, 0x00, 0x00, 0x00,
    0x00, 0x02, 0x04, 0x05, 0xb4, 0x04, 0x02, 0x08, 0x0a,
];

static mut CHAIN: [VirtioChainEntry; crate::constants::VIRTIO_ENTRY] = [VirtioChainEntry {
    idx: 0,
    addr: 0,
    len: 0,
    writeable: false,
};
    crate::constants::VIRTIO_ENTRY];

const VIRTIO_REG_MAGIC: u64 = 0x00;
const VIRTIO_REG_VERSION: u64 = 0x04;
const VIRTIO_REG_DEVICE_ID: u64 = 0x08;
const VIRTIO_REG_QUEUE_SEL: u64 = 0x30;
const VIRTQ_ENTRY_NUM: usize = 16;
const VIRTIO_REG_QUEUE_ALIGN: u64 = 0x3c;
const VIRTIO_REG_QUEUE_PFN: u64 = 0x40;
const VIRTIO_REG_QUEUE_NUM: u64 = 0x38;
const VIRTIO_NET_PADDR: crate::types::PaddrT = 0x10008000;
const VIRTIO_REG_QUEUE_NOTIFY: u64 = 0x50;
const VIRTIO_REG_DEVICE_STATUS: u64 = 0x70;
const VIRTIO_REG_DEVICE_CONFIG: u64 = 0x100;
const VIRTIO_STATUS_ACK: u32 = 1;
const VIRTIO_STATUS_DRIVER: u32 = 2;
const VIRTIO_STATUS_DRIVER_OK: u32 = 4;
const VIRTIO_STATUS_FEAT_OK: u32 = 8;
const VIRTIO_DEVICE_NET: u32 = 1;

static mut TX_VIRTQ: *mut Virtq = core::ptr::null_mut();
static mut RX_VIRTQ: *mut Virtq = core::ptr::null_mut();
static mut VIRTIO_NET_REQUEST: *mut crate::virtio_net::virtio_net_req = core::ptr::null_mut();

unsafe fn reg_read32(offset: u64) -> u32 {
    ((VIRTIO_NET_PADDR + offset) as *const u32).read_volatile()
}

unsafe fn reg_read64(offset: u64) -> u64 {
    ((VIRTIO_NET_PADDR + offset) as *const u64).read_volatile()
}

unsafe fn reg_write32(offset: u64, value: u32) {
    ((VIRTIO_NET_PADDR + offset) as *mut u32).write_volatile(value);
}

unsafe fn reg_fetch_and_or32(offset: u64, value: u32) {
    reg_write32(offset, reg_read32(offset) | value);
}

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
        let virtq_addr = unsafe {
            crate::paging::alloc_pages(
                crate::util::align_up(
                    core::mem::size_of::<Virtq>() as u64,
                    crate::constants::PAGE_SIZE as u64,
                ) / crate::constants::PAGE_SIZE as u64,
            )
        };

        let virtq = unsafe { (virtq_addr as u64 as *mut Virtq).as_mut().unwrap() };
        virtq.idx = idx;
        virtq.used_idx = unsafe {
            (&mut (virtq.used) as *const VirtqUsed as *const u8)
                .offset(core::mem::offset_of!(VirtqUsed, idx) as isize)
        } as *mut u16;

        unsafe {
            reg_write32(VIRTIO_REG_QUEUE_SEL, idx);
            reg_write32(VIRTIO_REG_QUEUE_NUM, VIRTQ_ENTRY_NUM as u32);
            reg_write32(VIRTIO_REG_QUEUE_ALIGN, 0);
            reg_write32(VIRTIO_REG_QUEUE_PFN, virtq_addr as u32);
        }

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

    pub fn recv(&mut self) -> Option<&[VirtioChainEntry]> {
        let mut chain_idx = 0;
        let mut desc_idx = 0;
        let mut avail_idx = self.avail.idx as usize;

        while self.used.ring[self.last_used_idx as usize].id != 0 {
            let used_elem = &self.used.ring[self.last_used_idx as usize];
            let mut desc = &self.desc[desc_idx];
            let addr = desc.addr;
            let mut len = desc.len;

            while desc.next != 0 {
                desc_idx = desc.next as usize;
                desc = &self.desc[desc_idx];
                len += desc.len;
            }

            unsafe {
                CHAIN[chain_idx] = VirtioChainEntry {
                    idx: 0,
                    addr,
                    len,
                    writeable: false,
                }
            };

            chain_idx += 1;
            self.last_used_idx += 1;
            avail_idx += 1;
        }

        if chain_idx == 0 {
            None
        } else {
            Some(unsafe { &CHAIN[0..chain_idx] })
        }
    }

    pub fn alloc_dmabuf(&mut self, paddr: &mut crate::types::PaddrT) -> Result<(), u32> {
        for i in 0..crate::constants::VIRTIO_ENTRY {
            if self.desc[i].len == 0 {
                self.desc[i].len = crate::constants::VIRTIO_ENTRY as u32;
                *paddr = self.desc[i].addr;
                return Ok(());
            }
        }
        Err(crate::constants::VIRTIO_ERR_NO_BUF)
    }
}

pub unsafe fn init() {
    assert!(reg_read32(VIRTIO_REG_MAGIC) == 0x74726976);
    assert!(reg_read32(VIRTIO_REG_VERSION) == 1);
    assert!(reg_read32(VIRTIO_REG_DEVICE_ID) == VIRTIO_DEVICE_NET);

    reg_write32(VIRTIO_REG_DEVICE_STATUS, 0);
    reg_fetch_and_or32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_ACK);
    reg_fetch_and_or32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_DRIVER);
    reg_fetch_and_or32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_FEAT_OK);

    let mut mac_addr: [u8; 6] = [0; 6];
    print!("mac addr: ");
    for j in 0..6 {
        mac_addr[j] = unsafe {
            core::ptr::read_volatile(
                (VIRTIO_NET_PADDR + VIRTIO_REG_DEVICE_CONFIG + (j as u64)) as *const u8,
            )
        };
        print!("{:02x}", mac_addr[j]);
        if j != 5 {
            print!(":");
        }
    }
    println!("");

    TX_VIRTQ = Virtq::new(0);
    RX_VIRTQ = Virtq::new(1);

    reg_write32(VIRTIO_REG_DEVICE_STATUS, VIRTIO_STATUS_DRIVER_OK);

    let virtio_net_request_paddr = crate::paging::alloc_pages(
        crate::util::align_up(
            core::mem::size_of::<crate::virtio_net::virtio_net_req>() as u64,
            crate::constants::PAGE_SIZE as u64,
        ) / crate::constants::PAGE_SIZE as u64,
    );

    VIRTIO_NET_REQUEST = virtio_net_request_paddr as *mut crate::virtio_net::virtio_net_req;
}

pub unsafe fn virtio_net_transmit(payload: &[u8]) -> Result<(), u32> {
    if payload.len() > crate::constants::VIRTIO_NET_MAX_PACKET_SIZE {
        return Err(crate::constants::VIRTIO_ERR_TOO_LARGE);
    }

    let mut req = crate::virtio_net::virtio_net_req {
        header: crate::virtio_net::virtio_net_hdr {
            flags: 0,
            gso_type: 0,
            hdr_len: 0,
            gso_size: 0,
            csum_start: 0,
            csum_offset: 0,
            num_buffers: 0,
        },
        payload: [0; crate::constants::VIRTIO_NET_MAX_PACKET_SIZE],
    };

    req.payload[..payload.len()].copy_from_slice(&payload[0..]);

    let mut paddr: crate::types::PaddrT = 0;
    TX_VIRTQ.as_mut().unwrap().alloc_dmabuf(&mut paddr).unwrap();

    let chain = [VirtioChainEntry {
        idx: 0,
        addr: paddr,
        len: core::mem::size_of::<crate::virtio_net::virtio_net_hdr>() as u32,
        writeable: false,
    }];

    TX_VIRTQ.as_mut().unwrap().send(&chain);

    Ok(())
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
    assert_eq!(unsafe { (*virtq).idx }, 0);
}
