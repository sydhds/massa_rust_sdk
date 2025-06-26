// #![no_std]
#![cfg_attr(not(test), no_std)]

/*
use lol_alloc::LeakingPageAllocator;
#[global_allocator]
static ALLOCATOR: LeakingPageAllocator = LeakingPageAllocator;

extern crate alloc;
use alloc::{
    // format,
    vec
};
*/

use massa_rust_sc::{generateEvent, get_data, print_msg, set_data};
// use core::ptr;
// use alloc::vec::Vec;

// use massa_rust_sc::__new;
// use massa_rust_sc::__new;

/*
// Module name provided by the host (Wasmer)
#[link(wasm_import_module = "massa")]
extern "C" {

    // External function signatures

    #[link_name = "assembly_script_generate_event"]
    fn generateEvent(event: i32) -> ();

    #[link_name = "assembly_script_set_data"]
    fn set_data(key: i32, value: i32) -> ();

    #[link_name = "assembly_script_get_data"]
    fn get_data(key: i32) -> i32;
}
*/

#[no_mangle]
extern "C" fn constructor() {
    /*
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
    */

    print_msg();

    // Storage set
    {
        let mut key: [u8; 4 + 24] = [0; 4 + 24];
        let key_size = 24u32;
        key[0..4].copy_from_slice(key_size.to_le_bytes().as_slice());
        // == "greeting_key"
        let key_str = [
            103u8, 0, 114, 0, 101, 0, 101, 0, 116, 0, 105, 0, 110, 0, 103, 0, 95, 0, 107, 0, 101,
            0, 121, 0,
        ];
        key[4..].copy_from_slice(&key_str);

        // == "hello"
        let mut value: [u8; 4 + 10] = [0; 4 + 10];
        let value_size = 10u32;
        value[0..4].copy_from_slice(value_size.to_le_bytes().as_slice());
        let value_str = [104, 0, 101, 0, 108, 0, 108, 0, 112, 0];
        value[4..].copy_from_slice(&value_str);

        unsafe {
            let key_ptr = key.as_mut_ptr().offset(4) as i32;
            let value_ptr = value.as_mut_ptr().offset(4) as i32;
            generateEvent(value_ptr);
            set_data(key_ptr, value_ptr);
        }
    }

    // End Storage set

    // Storage get
    {
        let mut key: [u8; 4 + 24] = [0; 4 + 24];
        let key_size = 24u32;
        key[0..4].copy_from_slice(key_size.to_le_bytes().as_slice());
        // == "greeting_key"
        let key_str = [
            103u8, 0, 114, 0, 101, 0, 101, 0, 116, 0, 105, 0, 110, 0, 103, 0, 95, 0, 107, 0, 101,
            0, 121, 0,
        ];
        key[4..].copy_from_slice(&key_str);
        unsafe {
            let key_ptr = key.as_mut_ptr().offset(4) as i32;
            generateEvent(key_ptr);
        }

        unsafe {
            let key_ptr = key.as_mut_ptr().offset(4); // as i32;
            let value_ptr = get_data(key_ptr as i32);
            generateEvent(value_ptr);
        };
    }
}

/*
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
*/

#[no_mangle]
extern "C" fn hello() -> *mut u8 {
    let mut key: [u8; 4 + 24] = [0; 4 + 24];
    let key_size = 24u32;
    key[0..4].copy_from_slice(key_size.to_le_bytes().as_slice());
    // == "greeting_key"
    let key_str = [
        103u8, 0, 114, 0, 101, 0, 101, 0, 116, 0, 105, 0, 110, 0, 103, 0, 95, 0, 107, 0, 101, 0,
        121, 0,
    ];
    key[4..].copy_from_slice(&key_str);

    let value_ptr = unsafe {
        let key_ptr = key.as_mut_ptr().offset(4); // as i32;
        let value_ptr = get_data(key_ptr as i32);
        value_ptr as *mut u8
    };

    value_ptr
}

/*
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // emit a wasm unreachable instruction if a panic occurs in our code
    core::arch::wasm32::unreachable()
}
*/

#[cfg_attr(not(test), panic_handler)]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // emit a wasm unreachable instruction if a panic occurs in our code
    core::arch::wasm32::unreachable()
}

#[cfg(test)]
mod tests {
    use core::ptr::slice_from_raw_parts;
    use super::*;
    use core::slice;

    #[test]
    #[no_mangle]
    fn test_unit_xax() {
        
        // Storage set
        
        let mut key: [u8; 4 + 24] = [0; 4 + 24];
        let key_size = 24u32;
        key[0..4].copy_from_slice(key_size.to_le_bytes().as_slice());
        // == "greeting_key"
        let key_str = [
            103u8, 0, 114, 0, 101, 0, 101, 0, 116, 0, 105, 0, 110, 0, 103, 0, 95, 0, 107, 0, 101,
            0, 121, 0,
        ];
        key[4..].copy_from_slice(&key_str);

        // == "hellw"
        let mut value: [u8; 4 + 10] = [0; 4 + 10];
        let value_size = 10u32;
        value[0..4].copy_from_slice(value_size.to_le_bytes().as_slice());
        let value_str = [104, 0, 101, 0, 108, 0, 108, 0, 119, 0];
        value[4..].copy_from_slice(&value_str);

        unsafe {
            let key_ptr = key.as_mut_ptr().offset(4) as i32;
            let value_ptr = value.as_mut_ptr().offset(4) as i32;
            generateEvent(value_ptr);
            set_data(key_ptr, value_ptr);
        }

        // End Storage set
        
        let res_ptr = hello();

        let res_size = unsafe {
            let res_size_ptr = res_ptr.offset(-4);
            let slice = slice::from_raw_parts(res_size_ptr, 4);
            u32::from_le_bytes(slice.try_into().unwrap())
        };

        assert_eq!(res_size, 10);

        let res_msg = unsafe {
            slice_from_raw_parts(res_ptr, res_size as usize).as_ref().unwrap()
        };

        let expected = [104, 0, 101, 0, 108, 0, 108, 0, 119, 0];
        assert_eq!(res_msg, expected);
        
    }
}
