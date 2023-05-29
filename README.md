# Windows Memory Access Crate
Working with Windows Official API Crate can be little overwhelming for some people, this Lib is created to make it easier for reading and writing to memory.

You can simply pass in the process PID  and be able to read from memory using u32 offsets. There is variety of functions to choose from, you want to read a u32, u16 or even a utf16 string, it is included, same goes for writing to memory locations.

If your plant is to read and write a lot to the same process, I recommend using the Windows Official API, as each function in this crate will auto open and close handles for every read/write situation. You Can get more performance using Windows API by opening a handle and keeping it open until you finihs all your reading/writing.

Disclaimer: This lib make use of unsafe memory manipulation functions, be sure to know what you are doing as manually manipulating memory can give unpredictable results. Reading and Writing to memory also require adminstrator role before you are able to read or write to memory.
