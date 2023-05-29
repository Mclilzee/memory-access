use std::ffi::c_void;
use windows::core::Error;
use windows::Win32::Foundation::{CloseHandle, GetLastError, HANDLE};
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_VM_READ};

// Reading an offset provide an empty buffer with amount of bytes to read into then convert to
// proper type and return.

pub fn read_u32(pid: u32, address_offset: u32) -> Result<u32, Error> {
    let handle = get_read_only_handle(pid);
    let mut buffer = [0u8; 4];

    let result = unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };

    unsafe { CloseHandle(handle) }.expect("Failed to close handle");

    if result.as_bool() {
        Ok(u32::from_le_bytes(buffer))
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn read_u16(pid: u32, address_offset: u32) -> Result<u16, Error> {
    let handle = get_read_only_handle(pid);
    let mut buffer = [0u8; 2];

    let result = unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };

    unsafe { CloseHandle(handle) }.expect("Failed to close handle");

    if result.as_bool() {
        Ok(u16::from_le_bytes(buffer))
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn read_u8(pid: u32, address_offset: u32) -> Result<u8, Error> {
    let handle = get_read_only_handle(pid);
    let mut buffer = [0u8; 1];

    let result = unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };

    unsafe { CloseHandle(handle) }.expect("Failed to close handle");

    if result.as_bool() {
        Ok(u8::from_le_bytes(buffer))
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn read_f32(pid: u32, address_offset: u32) -> Result<f32, Error> {
    let handle = get_read_only_handle(pid);
    let mut buffer = [0u8; 4];

    let result = unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };

    unsafe { CloseHandle(handle) }.expect("Failed to close handle");

    if result.as_bool() {
        Ok(f32::from_le_bytes(buffer))
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn read_utf16_string(pid: u32, address_offset: u32) -> Result<String, Error> {
    let handle = get_read_only_handle(pid);
    let mut buffer = [0u16; 100];

    let result = unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };

    unsafe { CloseHandle(handle) }.expect("Failed to close handle");

    if result.as_bool() {
        Ok(convert_utf16_array_to_string(buffer))
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

fn convert_utf16_array_to_string(buffer: [u16; 100]) -> String {
    let utf16_array = buffer
        .iter()
        .take_while(|&&c| c != 0)
        .cloned()
        .collect::<Vec<u16>>();

    String::from_utf16_lossy(&utf16_array[..])
}

pub fn get_read_only_handle(pid: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_VM_READ, true, pid) }.expect("Failed to open read access handle.")
}
