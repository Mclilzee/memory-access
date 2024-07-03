# Windows Memory Access Crate
Working with Windows Official API Crate can be little overwhelming for some people, this Lib is created to make it easier for reading and writing to memory.

You can simply pass in the process PID  and be able to read from memory using u32 offsets. There is variety of functions to choose from, you want to read a u32, u16 or even a utf16 string, it is included, same goes for writing to memory locations.

Disclaimer: This lib make use of unsafe memory manipulation functions, and I'm no expert, use at your own risk. Make sure to know what you are doing as manually manipulating memory can give unpredictable results. Reading and Writing to memory also require adminastrator role before you are able to read or write to memory.

## Examples

### Reading
requires a handle with read writes can be constructed using one of the Handle constructors `Handle::read_only` `Handle::full_access`.
```rs
use windows_memory_access::Handle;

fn main() {
    let pid: u32 = 50;
    let handle = Handle::read_only(pid).unwrap();

    let address_offset = 2050;
    let base = handle.read_u32(address_offset).unwrap() + 24;

    let base_offset = handle.read_u32(base).unwrap() + 48;
    let desired_value = handle.read_u8(base_offset).unwrap();

    handle.close().unwrap();

    println!("{desired_value}");
}
```

### Writing
requires a handle with write access or all access, can be constructed with Handle constructors `Handle::full_access`
```rs
use windows_memory_access::Handle;

fn main() {
    let pid: u32 = 2020;
    let handle: Handle = Handle::full_access(pid).unwrap();

    let address_offset = 5050;
    let desired_value: u32 = 202;
    handle.write_u32(address_offset, desired_value).unwrap();
    handle.close().unwrap();
    println!("{desired_value}");
}
```

Version: 0.5 update Revert back changes to return a result, as an error will only occur on the read memory and write memory but will not effect the returning result, in worst case the resturning result will always be a 0 as we init a 0 values bytes array then we modify it inside the windows api functions, if nothing changes the arrays will remain the same there for returning a 0.

Version: 0.6.0 update to clean up the API to let Handle struct Handles (pun intended) everything about writing and reading without relying on windows crate manually. We heavily make use of results to indicate a read , write failure.
