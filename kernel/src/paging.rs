pub static mut NEXT_PADDR: u64 = 0;

pub unsafe fn alloc_pages(n: u64) -> crate::types::PaddrT {
    let paddr = NEXT_PADDR;
    NEXT_PADDR += n * crate::constants::PAGE_SIZE;

    if NEXT_PADDR > core::ptr::addr_of!(crate::__free_ram_end) as crate::types::PaddrT {
        panic!("out of memory");
    }

    core::ptr::write_bytes(
        paddr as *mut u8,
        0,
        (n * crate::constants::PAGE_SIZE) as usize,
    );

    paddr
}

pub unsafe fn map_page(
    table2: crate::types::PaddrT,
    vaddr: crate::types::VaddrT,
    paddr: crate::types::PaddrT,
    flags: u64,
) {
    if vaddr % crate::constants::PAGE_SIZE != 0 {
        panic!("unaligned vaddr {:x}", vaddr);
    }

    if paddr % crate::constants::PAGE_SIZE != 0 {
        panic!("unaligned paddr {:x}", paddr);
    }

    let table2 = table2 as *mut u64;
    let vpn2 = ((vaddr >> 30) & 0x1ff) as isize;
    if *table2.offset(vpn2) & crate::constants::PAGE_V == 0 {
        let pt_addr = alloc_pages(1);
        table2
            .offset(vpn2)
            .write(((pt_addr / crate::constants::PAGE_SIZE) << 10) | crate::constants::PAGE_V);
    }

    let table1 = (*table2.offset(vpn2) << 2 & !0xfff) as *mut u64;
    let vpn1 = ((vaddr >> 21) & 0x1ff) as isize;
    if *table1.offset(vpn1) & crate::constants::PAGE_V == 0 {
        let pt_paddr = alloc_pages(1);
        table1
            .offset(vpn1)
            .write(((pt_paddr / crate::constants::PAGE_SIZE) << 10) | crate::constants::PAGE_V);
    }

    let table0 = (*table1.offset(vpn1) << 2 & !0xfff) as *mut u64;
    let vpn0 = ((vaddr >> 12) & 0x1ff) as isize;
    table0
        .offset(vpn0)
        .write(((paddr / crate::constants::PAGE_SIZE) << 10) | flags | crate::constants::PAGE_V);
}
