pub static mut RX_VIRTQ: *mut crate::virtio::Virtq = 0 as *mut crate::virtio::Virtq;
pub static mut TX_VIRTQ: *mut crate::virtio::Virtq = 0 as *mut crate::virtio::Virtq;
static mut RX_DMABUF: crate::virtio::Dmabuf = crate::virtio::Dmabuf {
    paddr: 0,
    vaddr: 0,
    entry_size: 0,
    num_entries: 0,
    used: [false; crate::constants::VIRTIO_ENTRY],
};
static mut TX_DMABUF: crate::virtio::Dmabuf = crate::virtio::Dmabuf {
    paddr: 0,
    vaddr: 0,
    entry_size: 0,
    num_entries: 0,
    used: [false; crate::constants::VIRTIO_ENTRY],
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct virtio_net_config {
    mac: [u8; 6],
    status: u16,
    max_vq_pairs: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct virtio_net_hdr {
    pub flags: u8,
    pub gso_type: u8,
    pub hdr_len: u16,
    pub gso_size: u16,
    pub csum_start: u16,
    pub csum_offset: u16,
    pub num_buffers: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct virtio_net_ctrl {
    pub class: u8,
    pub cmd: u8,
    pub data: [u8; 6],
    pub ack: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct virtio_net_ctrl_mac {
    pub entries: u32,
    pub mac: [u8; 6],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct virtio_net_ctrl_mq {
    pub virtqueue_pairs: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct virtio_net_req {
    pub header: virtio_net_hdr,
    pub payload: [u8; crate::constants::VIRTIO_NET_MAX_PACKET_SIZE],
}

impl virtio_net_req {
    pub fn transmit(&self, payload: &[u8]) -> Result<(), u32> {
        if payload.len() > crate::constants::VIRTIO_NET_MAX_PACKET_SIZE {
            return Err(crate::constants::VIRTIO_ERR_TOO_LARGE);
        }

        let mut req = virtio_net_req {
            header: virtio_net_hdr {
                flags: 0,
                gso_type: 0,
                hdr_len: 0,
                gso_size: 0,
                csum_start: 0,
                csum_offset: 0,
            },
            payload: [0; crate::constants::VIRTIO_NET_MAX_PACKET_SIZE],
        };

        req.payload[..payload.len()].copy_from_slice(&payload[0..]);

        let mut paddr: crate::types::PaddrT = 0;

        unsafe {
            TX_DMABUF.alloc_dmabuf(&mut paddr).unwrap();
        }

        let chain = [crate::virtio::VirtioChainEntry {
            idx: 0,
            addr: paddr,
            len: core::mem::size_of::<virtio_net_hdr>() as u32,
            writeable: false,
        }];

        unsafe {
            (*TX_VIRTQ).send(&chain);
        }

        Ok(())
    }
}

pub fn virtq_init() {
    unsafe {
        RX_VIRTQ = crate::virtio::Virtq::new(crate::constants::VIRTIO_NET_RX_QUEUE_IDX);
        TX_VIRTQ = crate::virtio::Virtq::new(crate::constants::VIRTIO_NET_TX_QUEUE_IDX);
    }
}
