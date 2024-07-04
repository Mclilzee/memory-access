use std::ffi::c_void;

use windows::{
    core::Error,
    Win32::{
        Foundation::HANDLE,
        System::Memory::{
            VirtualAllocEx, VirtualFreeEx, PAGE_PROTECTION_FLAGS, VIRTUAL_ALLOCATION_TYPE,
            VIRTUAL_FREE_TYPE,
        },
    },
};

pub fn virtual_alloc_ex(handle: HANDLE, size: usize) -> Result<u32, Error> {
    let v_type: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(4096);
    let protection: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(64);
    unsafe {
        let address = VirtualAllocEx(handle, None, size, v_type, protection);
        if address.is_null() {
            Err(Error::empty())
        } else {
            Ok(address as u32)
        }
    }
}

pub fn virtual_free_ex(handle: HANDLE, address: u32) -> Result<(), Error> {
    let free_type = VIRTUAL_FREE_TYPE(32768);
    unsafe { VirtualFreeEx(handle, address as *mut c_void, 0, free_type) }
}
