#![no_std]

use lol_alloc::LeakingPageAllocator;
#[global_allocator]
static ALLOCATOR: LeakingPageAllocator = LeakingPageAllocator;

extern crate alloc;
use alloc::vec;

#[link(wasm_import_module = "massa")]
extern "C" {

    // External function signatures

    #[link_name = "assembly_script_generate_event"]
    pub fn generateEvent(event: i32) -> ();

    #[link_name = "assembly_script_set_data"]
    pub fn set_data(key: i32, value: i32) -> ();

    #[link_name = "assembly_script_get_data"]
    pub fn get_data(key: i32) -> i32;
}

pub fn print_msg() {

    let mut buffer: [u8; 8] = [0; 8];
    let msg_size = 4u32;
    buffer[0..4].copy_from_slice(msg_size.to_le_bytes().as_slice());
    // == "Ab" | "A__X__"
    buffer[4] = 65;
    buffer[5] = 0;
    buffer[6] = 100;
    buffer[7] = 0;

    unsafe {
        let buffer_ptr = buffer
            .as_mut_ptr()
            .offset(4) as i32;
        generateEvent(buffer_ptr);
    }

}

#[no_mangle]
// extern "C" fn __new(size_ptr: *mut u8, id_ptr: *mut u8) -> *mut u8 {
extern "C" fn __new(size: usize, _id: i32) -> *mut u8 {
    // https://www.assemblyscript.org/runtime.html#interface
    // function __new(size: usize, id: u32): usize
    // https://github.com/AssemblyScript/assemblyscript/blob/main/std/assembly/rt/itcms.ts#L260
    // Note: id is defined as u32 in doc but as i32 in source code

    const HEADER_SIZE: usize = 20;
    let mut v = vec![0; HEADER_SIZE + size];
    // let mut v = vec![0; header_size + size];
    v[12..16].copy_from_slice(&[1, 0, 0, 0]);
    // v[12..16].copy_from_slice(&id_bytes);
    // v[16..header_size].copy_from_slice(&[32, 0, 0, 0]);
    v[16..HEADER_SIZE].copy_from_slice(&size.to_le_bytes());

    unsafe {
        v.leak().as_mut_ptr().offset(HEADER_SIZE as isize)
    }
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // emit a wasm unreachable instruction if a panic occurs in our code
    core::arch::wasm32::unreachable()
}
