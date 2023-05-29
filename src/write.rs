use std::ffi::c_void;
use windows::core::Error;
use windows::Win32::Foundation::{GetLastError, HANDLE};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;

pub fn write_u32(handle: HANDLE, address_offset: u32, value: u32) -> Result<(), Error> {
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

    if result.as_bool() {
        Ok(())
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn write_u16(handle: HANDLE, address_offset: u32, value: u16) -> Result<(), Error> {
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

    if result.as_bool() {
        Ok(())
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn write_u8(handle: HANDLE, address_offset: u32, value: u8) -> Result<(), Error> {
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

    if result.as_bool() {
        Ok(())
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn write_f32(handle: HANDLE, address_offset: u32, value: f32) -> Result<(), Error> {
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

    if result.as_bool() {
        Ok(())
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn write_utf16_string(handle: HANDLE, address_offset: u32, value: &str) -> Result<(), Error> {
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

    if result.as_bool() {
        Ok(())
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}
