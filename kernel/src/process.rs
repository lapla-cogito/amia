#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Process {
    pub pid: i64,
    pub state: i64,
    pub sp: *mut crate::types::VaddrT,
    pub page_table: crate::types::PaddrT,
    pub stack: [u8; 8192],
}

impl Process {
    pub const fn new() -> Self {
        Process {
            pid: 0,
            state: crate::constants::PROC_UNUSED,
            sp: core::ptr::null_mut(),
            page_table: 0,
            stack: [0; 8192],
        }
    }
}

static mut PROCS: [Process; crate::constants::PROCS_MAX] =
    [Process::new(); crate::constants::PROCS_MAX];
pub static mut IDLE_PROC: *mut Process = core::ptr::null_mut();
pub static mut CURRENT_PROC: *mut Process = core::ptr::null_mut();

#[naked]
#[no_mangle]
pub unsafe extern "C" fn switch_context(prev_sp: &*mut u64, next_sp: &*mut u64) {
    core::arch::asm!(
        "
        addi sp, sp, -13 * 8
        sd ra,  0  * 8(sp)
        sd s0,  1  * 8(sp)
        sd s1,  2  * 8(sp)
        sd s2,  3  * 8(sp)
        sd s3,  4  * 8(sp)
        sd s4,  5  * 8(sp)
        sd s5,  6  * 8(sp)
        sd s6,  7  * 8(sp)
        sd s7,  8  * 8(sp)
        sd s8,  9  * 8(sp)
        sd s9,  10 * 8(sp)
        sd s10, 11 * 8(sp)
        sd s11, 12 * 8(sp)
        sd sp, (a0)
        ld sp, (a1)
        ld ra,  0  * 8(sp)
        ld s0,  1  * 8(sp)
        ld s1,  2  * 8(sp)
        ld s2,  3  * 8(sp)
        ld s3,  4  * 8(sp)
        ld s4,  5  * 8(sp)
        ld s5,  6  * 8(sp)
        ld s6,  7  * 8(sp)
        ld s7,  8  * 8(sp)
        ld s8,  9  * 8(sp)
        ld s9,  10 * 8(sp)
        ld s10, 11 * 8(sp)
        ld s11, 12 * 8(sp)
        addi sp, sp, 13 * 8
        ret
        ",
        options(noreturn)
    );
}

pub unsafe fn create_process(img: *const crate::elf::ElfHeader) -> *mut Process {
    let mut proc = core::ptr::null_mut();
    let mut i = 0;

    for (ind, proc_iter) in PROCS.iter().enumerate().take(crate::constants::PROCS_MAX) {
        if proc_iter.state == crate::constants::PROC_UNUSED {
            i = ind;
            proc = &mut PROCS[i] as *mut Process;
            break;
        }
    }

    if !proc.is_null() {
        let sp = (&mut (*proc).stack as *mut [u8] as *mut u8)
            .add(core::mem::size_of_val(&(*proc).stack)) as *mut u64;
        *sp.sub(1) = 0; // s11
        *sp.sub(2) = 0; // s10
        *sp.sub(3) = 0; // s9
        *sp.sub(4) = 0; // s8
        *sp.sub(5) = 0; // s7
        *sp.sub(6) = 0; // s6
        *sp.sub(7) = 0; // s5
        *sp.sub(8) = 0; // s4
        *sp.sub(9) = 0; // s3
        *sp.sub(10) = 0; // s2
        *sp.sub(11) = 0; // s1
        *sp.sub(12) = 0; // s0
        *sp.sub(13) = shell_entry as usize as u64; // ra

        let page_table = crate::paging::alloc_pages(1);
        let mut paddr = core::ptr::addr_of!(crate::__kernel_base) as crate::types::PaddrT;

        while paddr < core::ptr::addr_of!(crate::__free_ram_end) as crate::types::PaddrT {
            crate::paging::map_page(
                page_table,
                paddr,
                paddr,
                crate::constants::PAGE_R | crate::constants::PAGE_W | crate::constants::PAGE_X,
            );
            paddr += crate::constants::PAGE_SIZE as u64;
        }

        if !img.is_null() {
            let asref = img.as_ref().unwrap();
            let count = asref.count_page();
            let page = crate::paging::alloc_pages(count as u64);
            asref.load(page);

            for i in 0..count {
                crate::paging::map_page(
                    page_table,
                    crate::constants::USER_BASE + (i * crate::constants::PAGE_SIZE) as u64,
                    page + (i * crate::constants::PAGE_SIZE) as u64,
                    crate::constants::PAGE_U
                        | crate::constants::PAGE_R
                        | crate::constants::PAGE_W
                        | crate::constants::PAGE_X,
                );
            }
        }

        (*proc).pid = i as i64 + 1;
        (*proc).state = crate::constants::PROC_READY;
        (*proc).sp = sp.sub(13);
        (*proc).page_table = page_table;

        proc
    } else {
        panic!("no free process slot");
    }
}

pub unsafe fn yield_proc() {
    let mut next = IDLE_PROC;
    for i in 0..crate::constants::PROCS_MAX {
        let proc = &mut PROCS[(CURRENT_PROC.as_ref().unwrap().pid as usize).wrapping_add(i)
            % crate::constants::PROCS_MAX] as *mut Process;

        if (*proc).state == crate::constants::PROC_READY && (*proc).pid > 0 {
            next = proc;
            break;
        }
    }

    if next == CURRENT_PROC {
        return;
    }

    let prev = CURRENT_PROC;
    CURRENT_PROC = next;

    core::arch::asm!(
        "
        sfence.vma
        csrw satp, {satp}
        sfence.vma
        ",
        satp = in(reg) (((*next).page_table / crate::constants::PAGE_SIZE as u64) | crate::constants::SATP_SV39),
    );

    crate::write_csr!(
        "sscratch",
        (&mut (*next).stack as *mut [u8] as *mut u8).add(core::mem::size_of_val(&(*next).stack))
            as *mut u64
    );

    switch_context(&(*prev).sp, &(*next).sp)
}

#[no_mangle]
unsafe extern "C" fn shell_entry() {
    core::arch::asm!(
        "
        csrw sepc, {sepc}
        csrw sstatus,{sstatus}
        sret
        ",
        sepc = in(reg) crate::constants::USER_BASE,
        sstatus = in(reg) crate::constants::SSTATUS_SPIE | crate::constants::SSTATUS_SUM,
        options(noreturn)
    );
}
