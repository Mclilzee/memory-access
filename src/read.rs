use std::ffi::c_void;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;

// Reading an offset provide an empty buffer with amount of bytes to read into then convert to
// proper type and return.
// requires a proper handle with reading rights, can use the windows_memory_access::handle::get_read_only_handle(pid); to
// simplify getting handles and windows_memory_access::handle::close_handle(handle); to close.

pub fn read_u32(handle: HANDLE, address_offset: u32) -> u32 {
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

    match result {
        Ok(_) => u32::from_le_bytes(buffer),
        Err(_) => 0,
    }
}

pub fn read_u16(handle: HANDLE, address_offset: u32) -> u16 {
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

    match result {
        Ok(_) => u16::from_le_bytes(buffer),
        Err(_) => 0,
    }
}

pub fn read_u8(handle: HANDLE, address_offset: u32) -> u8 {
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

    match result {
        Ok(_) => u8::from_le_bytes(buffer),
        Err(_) => 0,
    }
}

pub fn read_f32(handle: HANDLE, address_offset: u32) -> f32 {
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

    match result {
        Ok(_) => f32::from_le_bytes(buffer),
        Err(_) => 0.0,
    }
}

pub fn read_utf16_string(handle: HANDLE, address_offset: u32) -> String {
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

    match result {
        Ok(_) => {
            let utf16_array = buffer
                .into_iter()
                .take_while(|&c| c != 0)
                .collect::<Vec<u16>>();

            String::from_utf16_lossy(&utf16_array)
        }
        Err(_) => String::new(),
    }
}
