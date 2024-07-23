#[repr(C, packed)]
#[derive(Clone, Copy)]
struct virtio_net_config {
    mac: [u8; 6],
    status: u16,
    max_vq_pairs: u16,
    mtu: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct virtio_net_hdr {
    flags: u8,
    gso_type: u8,
    hdr_len: u16,
    gso_size: u16,
    csum_start: u16,
    csum_offset: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct virtio_net_req {
    header: virtio_net_hdr,
    payload: [u8; crate::constants::VIRTIO_NET_MAX_PACKET_SIZE],
}

pub fn read_macaddr(macaddr: &mut [u8; 6]) {
    let base = core::mem::offset_of!(virtio_net_config, mac);

    for i in 0..6 {
        macaddr[i] = unsafe {
            core::ptr::read_volatile((crate::constants::VIRTIO_NET_BASE + base + i) as *const u8)
        };
    }
}

pub fn transmit(payload: &[u8]) -> Result<(), u32> {
    if payload.len() > crate::constants::VIRTIO_NET_MAX_PACKET_SIZE {
        return Err(crate::constants::VIRTIO_ERR_TOO_LARGE);
    }

    todo!("transmit");

    Ok(())
}

pub fn init_device() {}
