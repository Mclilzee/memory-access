use std::ffi::c_void;
use windows::core::Error;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;

// requires a proper with write or all access rights handle, can use the windows_memory_access::handle::get_all_access_handle(pid); to
// simplify getting handles and windows_memory_access::handle::close_handle(handle); to close.

pub fn write_u8_bytes(handle: HANDLE, address_offset: u32, bytes: &[u8]) -> Result<(), Error> {
    unsafe {
        WriteProcessMemory(
            handle,
            address_offset as *const c_void,
            bytes.as_ptr() as *mut c_void,
            bytes.len(),
            None,
        )
    }
}

pub fn write_u16_bytes(handle: HANDLE, address_offset: u32, bytes: &[u16]) -> Result<(), Error> {
    unsafe {
        WriteProcessMemory(
            handle,
            address_offset as *const c_void,
            bytes.as_ptr() as *mut c_void,
            bytes.len(),
            None,
        )
    }
}
