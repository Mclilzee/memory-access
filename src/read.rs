use std::ffi::c_void;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;

// Reading an offset provide an empty buffer with amount of bytes to read into then convert to
// proper type and return.

pub fn read_u32(handle: HANDLE, address_offset: u32) -> u32 {
    let mut buffer = [0u8; 4];

    unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };

    u32::from_le_bytes(buffer)
}

pub fn read_u16(handle: HANDLE, address_offset: u32) -> u16 {
    let mut buffer = [0u8; 2];

    unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };

    u16::from_le_bytes(buffer)
}

pub fn read_u8(handle: HANDLE, address_offset: u32) -> u8 {
    let mut buffer = [0u8; 1];

    unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };

    u8::from_le_bytes(buffer)
}

pub fn read_f32(handle: HANDLE, address_offset: u32) -> f32 {
    let mut buffer = [0u8; 4];

    unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };

    f32::from_le_bytes(buffer)
}

pub fn read_utf16_string(handle: HANDLE, address_offset: u32) -> String {
    let mut buffer = [0u16; 100];

    unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
    };

    let utf16_array = buffer
        .iter()
        .take_while(|&&c| c != 0)
        .cloned()
        .collect::<Vec<u16>>();

    String::from_utf16_lossy(&utf16_array[..])
}
