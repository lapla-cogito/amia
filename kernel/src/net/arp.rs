const ARP_HDR_ETHER: usize = 0x0001;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    ArpRequest = 0x1,
    ArpResponse = 0x2,
}

#[derive(Debug)]
pub struct ArpHdr {
    pub htype: u16,
    pub ptype: u16,
    pub hlen: u8,
    pub plen: u8,
    pub op: Operation,
}

#[derive(Debug)]
pub struct ArpEther {
    pub hdr: ArpHdr,
    pub mac_src: crate::types::MacAddr,
    pub ip_src: crate::types::Ipv4Addr,
    pub mac_target: crate::types::MacAddr,
    pub ip_target: crate::types::Ipv4Addr,
}

impl ArpEther {
    pub fn request(
        src_eth: crate::types::MacAddr,
        src_ip: crate::types::Ipv4Addr,
        target_ip: crate::types::Ipv4Addr,
    ) -> Self {
        Self {
            hdr: ArpHdr {
                htype: ARP_HDR_ETHER as u16,
                ptype: crate::net::common::ARP_PRO_IP as u16,
                hlen: 6,
                plen: 4,
                op: Operation::ArpRequest,
            },
            mac_src: src_eth,
            ip_src: src_ip,
            mac_target: [0; 6],
            ip_target: target_ip,
        }
    }
}
