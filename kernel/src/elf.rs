#[repr(C, packed)]
#[derive(Debug)]
pub struct ElfHeader {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct ProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

impl ElfHeader {
    pub fn new(data: &[u8]) -> &Self {
        unsafe { &*(data.as_ptr() as *const ElfHeader) }
    }

    pub fn load(&self, paddr: crate::types::PaddrT) {
        let phdr = unsafe { (self as *const ElfHeader as *const u8).offset(self.e_phoff as isize) }
            as *const ProgramHeader;

        let mut paddr = paddr;
        for i in 0..self.e_phnum {
            let phdr = unsafe { phdr.offset(i as isize) };
            let off = unsafe { (*phdr).p_offset };
            let filesz = unsafe { (*phdr).p_filesz };

            let data = unsafe {
                core::slice::from_raw_parts(
                    (self as *const ElfHeader as *const u8).offset(off as isize),
                    filesz as usize,
                )
            };

            let pa_slice =
                unsafe { core::slice::from_raw_parts_mut(paddr as *mut u8, filesz as usize) };
            pa_slice.copy_from_slice(data);
            paddr += filesz;
        }
    }

    pub fn count_page(&self) -> usize {
        let phdr = unsafe { (self as *const ElfHeader as *const u8).offset(self.e_phoff as isize) }
            as *const ProgramHeader;
        let start_vaddr = unsafe { phdr.as_ref().unwrap().p_vaddr };
        let mut end_vaddr = start_vaddr;
        for i in 0..self.e_phnum as isize {
            let phdr = unsafe { phdr.offset(i).as_ref().unwrap() };
            let vaddr = phdr.p_vaddr;
            let memsz = phdr.p_memsz;

            if end_vaddr < vaddr + memsz {
                end_vaddr = vaddr + memsz;
            }
        }

        (crate::util::align_up(end_vaddr - start_vaddr, crate::constants::PAGE_SIZE)
            / crate::constants::PAGE_SIZE) as usize
    }
}
