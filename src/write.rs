use std::ffi::c_void;
use windows::core::Error;
use windows::Win32::Foundation::{CloseHandle, GetLastError, HANDLE};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

pub fn write_u32(pid: u32, address_offset: u32, value: u32) -> Result<(), Error> {
    let handle = get_all_access_handle(pid);
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

    unsafe { CloseHandle(handle) }.expect("Failed to close handle");

    if result.as_bool() {
        Ok(())
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn write_u16(pid: u32, address_offset: u32, value: u16) -> Result<(), Error> {
    let handle = get_all_access_handle(pid);
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

    unsafe { CloseHandle(handle) }.expect("Failed to close handle");

    if result.as_bool() {
        Ok(())
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn write_u8(pid: u32, address_offset: u32, value: u8) -> Result<(), Error> {
    let handle = get_all_access_handle(pid);
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

    unsafe { CloseHandle(handle) }.expect("Failed to close handle");

    if result.as_bool() {
        Ok(())
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn write_f32(pid: u32, address_offset: u32, value: f32) -> Result<(), Error> {
    let handle = get_all_access_handle(pid);
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

    unsafe { CloseHandle(handle) }.expect("Failed to close handle");

    if result.as_bool() {
        Ok(())
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn write_utf16_string(pid: u32, address_offset: u32, value: &str) -> Result<(), Error> {
    let handle = get_all_access_handle(pid);
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

    unsafe { CloseHandle(handle) }.expect("Failed to close handle");

    if result.as_bool() {
        Ok(())
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn get_all_access_handle(pid: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_ALL_ACCESS, true, pid) }
        .expect("Failed to open write access handle.")
}
