#![cfg_attr(not(test), no_std)]

extern crate alloc;
// rust crates
use alloc::vec::Vec;
use alloc::{format, vec};
// internal
use massa_rust_sc::{generateEvent, get_data, set_data};
// third-party
use utf16_lit::utf16;

// constants
const EXAMPLE: &[u16] = &utf16!("massa rust sdk");
const EXAMPLE_U8: &[u8] = bytemuck::must_cast_slice(EXAMPLE);

const KEY: &[u16] = &utf16!("greeting_key");
const KEY_U8: &[u8] = bytemuck::must_cast_slice(KEY);
const VALUE: &[u16] = &utf16!("hello");
const VALUE_U8: &[u8] = bytemuck::must_cast_slice(VALUE);
// end constants

#[no_mangle]
extern "C" fn constructor() {
    // Use generateEvent
    // Note: generateEvent requires an UTF16 encoded string as input
    unsafe {
        let mut msg: [u8; 4 + EXAMPLE_U8.len()] = [0; 4 + EXAMPLE_U8.len()];
        msg[0..4].copy_from_slice(EXAMPLE_U8.len().to_le_bytes().as_slice());
        msg[4..].copy_from_slice(EXAMPLE_U8);
        let ptr = msg.as_ptr().offset(4);
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
        // Define storage key
        // TODO: define const func?
        let mut key: [u8; 4 + KEY_U8.len()] = [0; 4 + KEY_U8.len()];
        key[0..4].copy_from_slice(KEY_U8.len().to_le_bytes().as_slice());
        key[4..].copy_from_slice(KEY_U8);

        // Define storage value
        let mut value: [u8; 4 + VALUE_U8.len()] = [0; 4 + VALUE_U8.len()];
        value[0..4].copy_from_slice(VALUE_U8.len().to_le_bytes().as_slice());
        value[4..].copy_from_slice(VALUE_U8);

        // Set our value in storage
        unsafe {
            let key_ptr = key.as_mut_ptr().offset(4) as i32;
            let value_ptr = value.as_mut_ptr().offset(4) as i32;
            // generateEvent(value_ptr);
            set_data(key_ptr, value_ptr);
        }
    }
}

#[no_mangle]
extern "C" fn hello() -> *mut u8 {
    // Define storage key
    // TODO: define const func?
    let mut key: [u8; 4 + KEY_U8.len()] = [0; 4 + KEY_U8.len()];
    key[0..4].copy_from_slice(KEY_U8.len().to_le_bytes().as_slice());
    key[4..].copy_from_slice(KEY_U8);

    let value_ptr = unsafe {
        let key_ptr = key.as_mut_ptr().offset(4); // as i32;
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
            slice_from_raw_parts(res_ptr, res_size as usize)
                .as_ref()
                .unwrap()
        };

        let expected = [104, 0, 101, 0, 108, 0, 108, 0, 119, 0];
        assert_eq!(res_msg, expected);
    }
}
