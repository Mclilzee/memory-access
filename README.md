# Windows Memory Access Crate
Working with Windows Official API Crate can be little overwhelming for some people, this Lib is create to make it easier for reading and writing to memory.

You can simply get a handle using process PID and be able to read from its memory using u32 offsets. There is variaty of functions to choose from weither you want to read a u32, u16 or even a utf16 string, it is included, same goes for writing to memory locations.

Disclaimer: This lib make use of unsafe memory manipulation functions, be sure to know what you are doing before you mess up with your memory, it will also require adminstrator role before you are able to read or write to memory.

```rs
pub fn get_player_address(pid: u32) -> u32 {
    let handle: HANDLE = get_handle(pid);

    let base = read_memory_u32(handle, address_constants::ADDRESS_BASE) + 28;
    let base_offset = read_memory_u32(handle, base) + 48;
    let player_address = read_memory_u32(handle, base_offset);

    unsafe { CloseHandle(handle) };
    player_address
}
```
