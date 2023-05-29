use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS, PROCESS_VM_READ};

pub fn get_read_only_handle(pid: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_VM_READ, true, pid) }.expect("Failed to open read access handle.")
}

pub fn get_all_access_handle(pid: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_ALL_ACCESS, true, pid) }
        .expect("Failed to open write access handle.")
}

pub fn close_handle(handle: HANDLE) {
    unsafe { CloseHandle(handle) }.expect("Failed to close handle.");
}
