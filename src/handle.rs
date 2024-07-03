use super::read;
use windows::core::Error;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS, PROCESS_VM_READ};

enum HandleType {
    ReadOnly,
    FullAccess,
}

pub struct Handle {
    pub pid: u32,
    pub handle: HANDLE,
    h_type: HandleType,
}

impl Handle {
    pub fn read_only(pid: u32) -> Result<Handle, Error> {
        unsafe {
            OpenProcess(PROCESS_VM_READ, true, pid).map(|handle| Self {
                handle,
                pid,
                h_type: HandleType::ReadOnly,
            })
        }
    }

    pub fn full_access(pid: u32) -> Result<Handle, Error> {
        unsafe {
            OpenProcess(PROCESS_ALL_ACCESS, true, pid).map(|handle| Self {
                handle,
                pid,
                h_type: HandleType::FullAccess,
            })
        }
    }

    pub fn close(self) -> Result<(), Error> {
        unsafe { CloseHandle(self.handle) }
    }

    pub fn read_u32(&self, address_offset: u32) -> Result<u32, Error> {
        read::u32_bytes(self.handle, address_offset).map(u32::from_le_bytes)
    }

    pub fn read_i32(&self, address_offset: u32) -> Result<i32, Error> {
        read::u32_bytes(self.handle, address_offset).map(i32::from_le_bytes)
    }

    pub fn read_u16(&self, address_offset: u32) -> Result<u16, Error> {
        read::u16_bytes(self.handle, address_offset).map(u16::from_le_bytes)
    }

    pub fn read_i16(&self, address_offset: u32) -> Result<i16, Error> {
        read::u16_bytes(self.handle, address_offset).map(i16::from_le_bytes)
    }

    pub fn read_u8(&self, address_offset: u32) -> Result<u8, Error> {
        read::u8_bytes(self.handle, address_offset).map(|b| b[0])
    }

    pub fn read_i8(&self, address_offset: u32) -> Result<i8, Error> {
        read::u8_bytes(self.handle, address_offset).map(i8::from_le_bytes)
    }
}
