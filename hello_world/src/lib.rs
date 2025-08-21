#![cfg_attr(not(test), no_std)]

extern crate alloc;
// rust crates
use alloc::vec::Vec;
use alloc::{format, vec};
// internal
use massa_rust_sc::{generateEvent, string_to_as_array, to_as_array, get_data, set_data};
// third-party
use utf16_lit::utf16;

// constants

const EXAMPLE: &[u8] = string_to_as_array!("massa_rust_sdk");
const KEY: &[u8] = string_to_as_array!("greeting_key");
const VALUE: &[u8] = string_to_as_array!("hello");

// end constants

#[no_mangle]
extern "C" fn constructor() {

    // Use generateEvent
    // Note: generateEvent requires an UTF16 encoded string as input
    unsafe {
        let ptr = EXAMPLE.as_ptr().offset(4);
        generateEvent(ptr as i32);
    }

    // Use generateEvent but with dynamic data (dynamic Rust string)
    let msg = format!("hello there {}!!", 42);
    unsafe {
        let msg_utf16 = msg.encode_utf16().collect::<Vec<u16>>();
        let msg_utf16_slice = msg_utf16.as_slice();
        let msg_utf8: &[u8] = bytemuck::cast_slice(msg_utf16_slice);

        let mut msg_final = vec![0; 4 + msg_utf8.len()];
        msg_final[0..4].copy_from_slice(msg_utf8.len().to_le_bytes().as_slice());
        msg_final[4..].copy_from_slice(msg_utf8);
        let ptr = msg_final.as_ptr().offset(4);
        generateEvent(ptr as i32);
    }

    // Storage set
    {
        // Set our value in smart contract storage
        unsafe {
            let key_ptr = KEY.as_ptr().offset(4) as i32;
            let value_ptr = VALUE.as_ptr().offset(4) as i32;
            // generateEvent(value_ptr);
            set_data(key_ptr, value_ptr);
        }
    }
}

#[no_mangle]
extern "C" fn hello() -> *mut u8 {


    #[allow(clippy::let_and_return)]
    let value_ptr = unsafe {
        let key_ptr = KEY.as_ptr().offset(4); // as i32;
        let value_ptr = get_data(key_ptr as i32);
        value_ptr as *mut u8
    };

    value_ptr
}

#[cfg_attr(not(test), panic_handler)]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // emit a wasm unreachable instruction if a panic occurs in our code
    core::arch::wasm32::unreachable()
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr::slice_from_raw_parts;
    use core::slice;

    #[test]
    #[no_mangle]
    fn __MASSA_RUST_SDK_UNIT_TEST_hello_1() {

        // Storage set (before calling hello())
        const T_KEY: &[u8] = string_to_as_array!("greeting_key");
        const T_VALUE: &[u8] = string_to_as_array!("hellw");

        unsafe {
            let key_ptr = T_KEY.as_ptr().offset(4) as i32;
            let value_ptr = T_VALUE.as_ptr().offset(4) as i32;
            generateEvent(value_ptr);
            set_data(key_ptr, value_ptr);
        }

        // Now call hello()

        let res_ptr = hello();

        let res_size = unsafe {
            let res_size_ptr = res_ptr.offset(-4);
            let slice = slice::from_raw_parts(res_size_ptr, 4);
            u32::from_le_bytes(slice.try_into().unwrap())
        };

        assert_eq!(res_size, 10);

        let res_msg = unsafe {
            slice_from_raw_parts(res_ptr, res_size as usize)
                .as_ref()
                .unwrap()
        };

        // let expected = [104, 0, 101, 0, 108, 0, 108, 0, 119, 0];
        let expected = bytemuck::must_cast_slice(&utf16!("hellw"));
        assert_eq!(res_msg, expected);
    }
}
