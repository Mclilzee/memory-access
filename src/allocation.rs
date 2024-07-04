use windows::Win32::{
    Foundation::HANDLE,
    System::Memory::{VirtualAllocEx, PAGE_PROTECTION_FLAGS, VIRTUAL_ALLOCATION_TYPE},
};

pub fn virtual_alloc_ex(handle: HANDLE, size: usize) -> u32 {
    let v_type: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(4096);
    let protection: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(64);
    unsafe { VirtualAllocEx(handle, None, size, v_type, protection) as u32 }
}

pub fn virtual_free_ex(handle: HANDLE, address: u32) {}
