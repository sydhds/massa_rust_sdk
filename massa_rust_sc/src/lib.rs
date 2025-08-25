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

    #[link_name = "assembly_script_has_data"]
    pub fn has_data(key: i32) -> bool;
}

#[no_mangle]
extern "C" fn __new(size: usize, _id: i32) -> *mut u8 {
    // https://www.assemblyscript.org/runtime.html#interface
    // function __new(size: usize, id: u32): usize
    // https://github.com/AssemblyScript/assemblyscript/blob/main/std/assembly/rt/itcms.ts#L260
    // Note: id is defined as u32 in doc but as i32 in source code

    const HEADER_SIZE: usize = 20;
    let mut v = vec![0; HEADER_SIZE + size];
    v[12..16].copy_from_slice(&[1, 0, 0, 0]);
    v[16..HEADER_SIZE].copy_from_slice(&size.to_le_bytes());

    unsafe {
        v.leak().as_mut_ptr().add(HEADER_SIZE)
    }
}

pub const fn to_as_array<const N: usize>(v: &[u8]) -> [u8; N] {
    let mut dst: [u8; N] = [0u8; N];
    let (a1, a2) = dst.split_at_mut(4);
    a1.copy_from_slice((v.len() as u32).to_le_bytes().as_slice());
    a2.copy_from_slice(v);
    dst
}

#[macro_export]
macro_rules! string_to_as_array {
    ($key:expr) => {{
        const K__: &[u16] = &utf16!($key);
        const K_U8__: &[u8] = bytemuck::must_cast_slice(K__);
        const N__: usize = K_U8__.len();
        to_as_array::<{N__ + 4}>(K_U8__).as_slice()
    }};
}



/*
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // emit a wasm unreachable instruction if a panic occurs in our code
    core::arch::wasm32::unreachable()
}
*/
