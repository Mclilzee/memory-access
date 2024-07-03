use std::ffi::c_void;
use windows::core::Error;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;

// Reading an offset provide an empty buffer with amount of bytes to read into then convert to
// proper type and return.
// requires a proper handle with reading rights, can use the windows_memory_access::handle::get_read_only_handle(pid); to
// simplify getting handles and windows_memory_access::handle::close_handle(handle); to close.

pub fn read_u32(handle: HANDLE, address_offset: u32) -> Result<u32, Error> {
    let mut buffer = [0u8; 4];

    unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )?
    };

    Ok(u32::from_le_bytes(buffer))
}

pub fn read_u16(handle: HANDLE, address_offset: u32) -> Result<u16, Error> {
    let mut buffer = [0u8; 2];

    unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )?
    };

    Ok(u16::from_le_bytes(buffer))
}

pub fn read_u8(handle: HANDLE, address_offset: u32) -> Result<u8, Error> {
    let mut buffer = [0u8; 1];

    unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )?
    };

    Ok(u8::from_le_bytes(buffer))
}

pub fn read_f32(handle: HANDLE, address_offset: u32) -> Result<f32, Error> {
    let mut buffer = [0u8; 4];

    unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )?
    };

    Ok(f32::from_le_bytes(buffer))
}

pub fn read_utf16_string(handle: HANDLE, address_offset: u32) -> Result<String, Error> {
    let mut buffer = [0u16; 100];

    unsafe {
        ReadProcessMemory(
            handle,
            address_offset as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        )
        .map(|_| {
            let utf16_array = buffer
                .into_iter()
                .take_while(|&c| c != 0)
                .collect::<Vec<u16>>();

            String::from_utf16_lossy(&utf16_array)
        })
    }
}
