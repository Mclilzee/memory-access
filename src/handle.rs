use windows::core::Error;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS, PROCESS_VM_READ};

pub fn open_read_only_handle(pid: u32) -> Result<HANDLE, Error> {
    unsafe { OpenProcess(PROCESS_VM_READ, true, pid) }
}

pub fn open_all_access_handle(pid: u32) -> Result<HANDLE, Error> {
    unsafe { OpenProcess(PROCESS_ALL_ACCESS, true, pid) }
}

pub fn close_handle(handle: HANDLE) -> Result<(), Error> {
    unsafe { CloseHandle(handle) }
}
