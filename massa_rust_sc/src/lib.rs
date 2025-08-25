#![no_std]

// Massa blockchain only accept Wasm without the reference-types and multivalue features
// According to this blog post: https://blog.rust-lang.org/2024/09/24/webassembly-targets-change-in-default-target-features/
// this is the default for Rust 1.82+
// So need to compile using Rust nightly (see blog rust-lang post)
// See also:
// https://github.com/rust-lang/rust/issues/128475
// https://github.com/rust-lang/rust/pull/128511

mod as_slice;
mod as_vec;
mod memory;
mod owner;
mod context;

use lol_alloc::LeakingPageAllocator;
#[global_allocator]
static ALLOCATOR: LeakingPageAllocator = LeakingPageAllocator;

extern crate alloc;
use crate::memory::AsMemoryModel;
use alloc::vec;

// export
pub use as_slice::{to_as_array, AsArray, AsSlice};
pub use as_vec::AsVec;

#[link(wasm_import_module = "massa")]
extern "C" {

    // External function signatures

    /// Generate an event in the blockchain
    ///
    /// * event: a pointer to an utf-16 string (prefixed with array size, see [string_to_as_array!](string_to_as_array!))
    #[link_name = "assembly_script_generate_event"]
    pub fn assembly_script_generate_event(event: i32) -> ();

    /// Store a value in smart contract storage
    ///
    /// * key: a pointer to a byte slice (prefixed with array size)
    /// * value: a pointer to a byte slice (prefixed with array size)
    #[link_name = "assembly_script_set_data"]
    pub fn assembly_script_set_data(key: i32, value: i32) -> ();

    /// Get a value stored in smart contract storage
    ///
    /// * key: a pointer to a byte slice (prefixed with array size)
    ///
    /// Return: a pointer to a byte slice (prefixed with array size)
    #[link_name = "assembly_script_get_data"]
    pub fn assembly_script_get_data(key: i32) -> i32;

    /// Check if a value is stored in smart contract storage
    ///
    /// * key: a pointer to a byte slice (prefixed with array size)
    ///
    /// Return: a boolean value
    #[link_name = "assembly_script_has_data"]
    pub fn assembly_script_has_data(key: i32) -> bool;

    #[link_name = "assembly_script_caller_has_write_access"]
    pub fn assembly_script_caller_has_write_access() -> bool;

    #[link_name = "assembly_script_get_call_stack"]
    pub fn assembly_script_get_call_stack() -> i32;
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

    unsafe { v.leak().as_mut_ptr().add(HEADER_SIZE) }
}

#[no_mangle]
extern "C" fn __pin(ptr: usize) -> usize {
    // https://www.assemblyscript.org/runtime.html#interface
    // function __pin(ptr: usize): usize
    // https://github.com/AssemblyScript/assemblyscript/blob/main/std/assembly/rt/itcms.ts#L334
    ptr
}

/*
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // emit a wasm unreachable instruction if a panic occurs in our code
    core::arch::wasm32::unreachable()
}
*/

pub fn generate_event<T: AsMemoryModel>(event: T) {
    unsafe {
        assembly_script_generate_event(event.as_ptr_data());
    }
}

pub fn set_data<T: AsMemoryModel, U: AsMemoryModel>(key: T, value: U) {
    unsafe {
        assembly_script_set_data(key.as_ptr_data(), value.as_ptr_data());
    }
}

pub fn get_data<T: AsMemoryModel>(key: T) -> i32 {
    unsafe { assembly_script_get_data(key.as_ptr_data()) }
}

pub fn has_data<T: AsMemoryModel>(key: T) -> bool {
    unsafe { assembly_script_has_data(key.as_ptr_data()) }
}

pub fn caller_has_write_access() -> bool {
    unsafe { assembly_script_caller_has_write_access() }
}
