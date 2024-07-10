mod allocation;
mod read;
mod threading;
mod write;

use windows::core::{Error, Free};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS, PROCESS_VM_READ};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Handle {
    pub handle: HANDLE,
}

impl Handle {
    pub fn read_only(pid: u32) -> Result<Handle, Error> {
        unsafe { OpenProcess(PROCESS_VM_READ, true, pid).map(|handle| Self { handle }) }
    }

    pub fn full_access(pid: u32) -> Result<Handle, Error> {
        unsafe { OpenProcess(PROCESS_ALL_ACCESS, true, pid).map(|handle| Self { handle }) }
    }

    pub fn read_u32(&self, offset: u32) -> Result<u32, Error> {
        read::u32_bytes(self.handle, offset).map(u32::from_le_bytes)
    }

    pub fn read_i32(&self, offset: u32) -> Result<i32, Error> {
        read::u32_bytes(self.handle, offset).map(i32::from_le_bytes)
    }

    pub fn read_u16(&self, offset: u32) -> Result<u16, Error> {
        read::u16_bytes(self.handle, offset).map(u16::from_le_bytes)
    }

    pub fn read_i16(&self, offset: u32) -> Result<i16, Error> {
        read::u16_bytes(self.handle, offset).map(i16::from_le_bytes)
    }

    pub fn read_u8(&self, offset: u32) -> Result<u8, Error> {
        read::u8_bytes(self.handle, offset).map(|b| b[0])
    }

    pub fn read_i8(&self, offset: u32) -> Result<i8, Error> {
        read::u8_bytes(self.handle, offset).map(i8::from_le_bytes)
    }

    pub fn read_u16_string(&self, offset: u32) -> Result<String, Error> {
        read::utf16_string(self.handle, offset)
    }

    pub fn write_u32(&self, offset: u32, value: u32) -> Result<(), Error> {
        let bytes = value.to_le_bytes();
        write::write_u8_bytes(self.handle, offset, &bytes)
    }

    pub fn write_u16(handle: HANDLE, offset: u32, value: u16) -> Result<(), Error> {
        let bytes = value.to_le_bytes();
        write::write_u8_bytes(handle, offset, &bytes)
    }

    pub fn write_u8(&self, offset: u32, value: u8) -> Result<(), Error> {
        let bytes = value.to_le_bytes();
        write::write_u8_bytes(self.handle, offset, &bytes)
    }

    pub fn write_bytes(&self, offset: u32, bytes: &[u8]) -> Result<(), Error> {
        write::write_u8_bytes(self.handle, offset, bytes)
    }

    pub fn write_f32(&self, offset: u32, value: f32) -> Result<(), Error> {
        let bytes = value.to_le_bytes();
        write::write_u8_bytes(self.handle, offset, &bytes)
    }

    pub fn write_utf16_string(&self, offset: u32, value: &str) -> Result<(), Error> {
        let bytes = value.encode_utf16().collect::<Vec<u16>>();
        write::write_u16_bytes(self.handle, offset, &bytes)
    }

    pub fn virtual_alloc_ex(&self, size: usize) -> Result<VirtualAllocEx, Error> {
        let address = allocation::virtual_alloc_ex(self.handle, size)?;
        Ok(VirtualAllocEx {
            address,
            handle: self.handle,
        })
    }

    pub fn create_remote_thread(&self, address: u32) -> Result<ThreadHandle, Error> {
        threading::create_remote_thread(self.handle, address).map(|handle| ThreadHandle { handle })
    }
}

pub struct VirtualAllocEx {
    pub address: u32,
    handle: HANDLE,
}

impl Drop for VirtualAllocEx {
    fn drop(&mut self) {
        allocation::virtual_free_ex(self.handle, self.address).ok();
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { self.handle.free() };
    }
}

pub struct ThreadHandle {
    handle: HANDLE,
}

impl ThreadHandle {
    pub fn wait(&self) -> Result<(), Error> {
        let result = threading::wait_for_single_object(self.handle);
        if result == 0 {
            Ok(())
        } else {
            Err(Error::empty())
        }
    }
}

impl Drop for ThreadHandle {
    fn drop(&mut self) {
        unsafe { self.handle.free() };
    }
}
