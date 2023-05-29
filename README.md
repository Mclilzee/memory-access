# Windows Memory Access Crate
Requires: <a href="https://crates.io/crates/windows">Windows API Crate Dependencies</a> for Opening and Closing handles

Adding required dependencies:
```toml
[dependencies]
windows = {version = "x.x.x", features = ["Win32_System_Threading", "Win32_Foundation"]}
```
Working with Windows Official API Crate can be little overwhelming for some people, this Lib is created to make it easier for reading and writing to memory.

You can simply pass in the process PID  and be able to read from memory using u32 offsets. There is variety of functions to choose from, you want to read a u32, u16 or even a utf16 string, it is included, same goes for writing to memory locations.

If your plant is to read and write a lot to the same process, I recommend using the Windows Official API, as each function in this crate will auto open and close handles for every read/write situation. You Can get more performance using Windows API by opening a handle and keeping it open until you finihs all your reading/writing.

Disclaimer: This lib make use of unsafe memory manipulation functions, be sure to know what you are doing as manually manipulating memory can give unpredictable results. Reading and Writing to memory also require adminstrator role before you are able to read or write to memory.

## Examples

### Reading
requires a handle with read acess, can be found in `windows_memory_access::handle::get_read_only_handle(pid);`
```rs
use memory_access::read::read_u32;
use memory_access::handle;
use windows::Win32::Foundation::HANDLE;



fn read_u32_value(pid: u32, address_offset: u32) -> u32 {
    let handle: HANDLE = handle::get_read_only_handle(pid);

    let base = read_u32(handle, address_offset) + 28;
    let base_offset = read_u32(handle, base) + 48;
    let desired_value = read_u32(handle, base_offset);

    handle::close_handle(handle);

    desired_value
}
```

### Writing
requires a handle with write access or all access, can be found in `windows_memory_access::handle:get_all_access_handle(pid);`
```rs
use memory_access::read::read_u32;
use memory_access::handle;
use windows::Win32::Foundation::HANDLE;



fn write_u32_value(pid: u32, address_offset: u32) -> u32 {
    let handle: HANDLE = handle::get_all_access_handle(pid);

    let desired_value: u32 = 202;

    write_u32(handle, address_offset, desired_value);

    handle::close_handle(handle);
}
```

Version: 0.5 update Revert back changes to return a result, as an error will only occur on the read memory and write memory but will not effect the returning result, in worst case the resturning result will always be a 0 as we init a 0 values bytes array then we modify it inside the windows api functions, if nothing changes the arrays will remain the same there for returning a 0.
