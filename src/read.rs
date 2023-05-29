use std::ffi::c_void;
use windows::core::Error;
use windows::Win32::Foundation::{GetLastError, HANDLE};
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;

// Reading an offset provide an empty buffer with amount of bytes to read into then convert to
// proper type and return.

pub fn read_u32(handle: HANDLE, address_offset: u32) -> Result<u32, Error> {
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

    if result.as_bool() {
        Ok(u32::from_le_bytes(buffer))
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn read_u16(handle: HANDLE, address_offset: u32) -> Result<u16, Error> {
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

    if result.as_bool() {
        Ok(u16::from_le_bytes(buffer))
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn read_u8(handle: HANDLE, address_offset: u32) -> Result<u8, Error> {
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

    if result.as_bool() {
        Ok(u8::from_le_bytes(buffer))
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn read_f32(handle: HANDLE, address_offset: u32) -> Result<f32, Error> {
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

    if result.as_bool() {
        Ok(f32::from_le_bytes(buffer))
    } else {
        Err(unsafe { GetLastError() }.into())
    }
}

pub fn read_utf16_string(handle: HANDLE, address_offset: u32) -> Result<String, Error> {
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
