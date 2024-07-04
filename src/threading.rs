use std::ffi::c_void;

use windows::{
    core::Error,
    Win32::{Foundation::HANDLE, System::Threading::CreateRemoteThread},
};

pub fn create_remote_thread(handle: HANDLE, address: u32) -> Result<HANDLE, Error> {
    unsafe {
        let thread_proc = Some(std::mem::transmute::<
            *const c_void,
            unsafe extern "system" fn(*mut c_void) -> u32,
        >(address as *const c_void));
        CreateRemoteThread(handle, None, 0, thread_proc, None, 0, None)
    }
}
