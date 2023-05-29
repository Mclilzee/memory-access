use std::ffi::c_void;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;

// requires a proper with write or all access rights handle, can use the windows_memory_access::handle::get_all_access_handle(pid); to
// simplify getting handles and windows_memory_access::handle::close_handle(handle); to close.

pub fn write_u32(handle: HANDLE, address_offset: u32, value: u32) {
    let buffer = value.to_le_bytes();

    unsafe {
        WriteProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };
}

pub fn write_u16(handle: HANDLE, address_offset: u32, value: u16) {
    let buffer = value.to_le_bytes();

    unsafe {
        WriteProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };
}

pub fn write_u8(handle: HANDLE, address_offset: u32, value: u8) {
    let buffer = value.to_le_bytes();

    let result = unsafe {
        WriteProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };
}

pub fn write_f32(handle: HANDLE, address_offset: u32, value: f32) {
    let buffer = value.to_le_bytes();

    let result = unsafe {
        WriteProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };
}

pub fn write_utf16_string(handle: HANDLE, address_offset: u32, value: &str) {
    let buffer = value.encode_utf16().collect::<Vec<u16>>();

    let result = unsafe {
        WriteProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };
}
