
use std::ffi::c_void;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::PROCESS_VM_READ;

pub fn read_memory_u32(handle: HANDLE, address: u32) -> u32 {
    let mut buffer = [0u8; 4];

    unsafe {
        ReadProcessMemory(
            handle,
            address as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        );
    }

    u32::from_le_bytes(buffer)
}

pub fn read_memory_u16(handle: HANDLE, address: u32) -> u16 {
    let mut buffer = [0u8; 2];

    unsafe {
        ReadProcessMemory(
            handle,
            address as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        );
    }

    u16::from_le_bytes(buffer)
}

pub fn read_memory_u8(handle: HANDLE, address: u32) -> u8 {
    let mut buffer = [0u8; 1];

    unsafe {
        ReadProcessMemory(
            handle,
            address as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        );
    }

    buffer[0]
}

pub fn read_memory_f32(handle: HANDLE, address: u32) -> f32 {
    let mut buffer = [0u8; 4];

    unsafe {
        ReadProcessMemory(
            handle,
            address as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        );
    }

    f32::from_le_bytes(buffer)
}

pub fn read_memory_utf16_string(handle: HANDLE, address: u32) -> String {
    let mut buffer = [0u16; 100];
    unsafe {
        ReadProcessMemory(
            handle,
            address as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len(),
            None,
        );
    }

    let utf16_array = buffer
        .iter()
        .take_while(|&&c| c != 0)
        .cloned()
        .collect::<Vec<u16>>();

    String::from_utf16_lossy(&utf16_array[..])
}

pub fn get_handle(pid: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_VM_READ, true, pid) }.expect("Error opening handle")
}
