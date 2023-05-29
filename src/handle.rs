use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::{PROCESS_VM_READ, PROCESS_VM_WRITE};

pub fn get_read_handle(pid: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_VM_READ, true, pid) }.expect("Error opening handle")
}

pub fn get_write_handle(pid: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_VM_WRITE, true, pid) }.expect("Error opening handle")
}
