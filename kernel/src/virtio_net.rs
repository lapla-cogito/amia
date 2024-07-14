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
