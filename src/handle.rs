use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::{PROCESS_VM_READ, PROCESS_VM_WRITE};

pub fn get_read_handle(pid: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_VM_READ, true, pid) }.expect("Failed to open read access handle.")
}

pub fn get_write_handle(pid: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_VM_WRITE, true, pid) }
        .expect("Failed to open write access handle.")
}

pub fn close_handle(handle: HANDLE) {
    unsafe { CloseHandle(handle) }.expect("Failed to close handle.");
}
