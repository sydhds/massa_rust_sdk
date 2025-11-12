#![cfg_attr(not(test), no_std)]

extern crate alloc;
// rust crates
use alloc::format;
// internal
use massa_rust_sc::{
    generate_event, get_data, is_deploying_contract, set_data,
    to_as_array, to_as_slice,
    AsSlice, AsVec
};
// third-party
use utf16_lit::utf16;

// constants

const EXAMPLE: AsSlice<u8> = to_as_slice!("massa_rust_sdk");
const KEY: AsSlice<u8> = to_as_slice!("greeting_key");
const VALUE: AsSlice<u8> = to_as_slice!("hello");

// end constants

#[no_mangle]
extern "C" fn constructor() {

    assert!(is_deploying_contract());

    // Use generateEvent
    // Note: generateEvent requires an UTF16 encoded string as input
    generate_event(EXAMPLE);

    // Use generateEvent but with dynamic data (dynamic Rust string)
    let msg = format!("hello there {}!!", 42);
    let msg_utf16 = msg.encode_utf16().collect::<AsVec<u16>>();
    generate_event(msg_utf16);

    // Storage set
    {
        // Set our value in smart contract storage
        set_data(KEY, VALUE);
    }
}

#[no_mangle]
extern "C" fn hello() -> *const u8 {
    let ptr = get_data(KEY);
    ptr as *const u8
}

#[cfg_attr(not(test), panic_handler)]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // emit a wasm unreachable instruction if a panic occurs in our code
    core::arch::wasm32::unreachable()
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::Deref;
    use massa_rust_sc::{has_data, AsSlice};

    #[test]
    #[no_mangle]
    fn __MASSA_RUST_SDK_UNIT_TEST_hello_1() {
        // Storage set (before calling hello())
        const T_KEY: AsSlice<u8> = to_as_slice!("greeting_key");
        const T_VALUE: AsSlice<u8> = to_as_slice!("hellw");

        generate_event(T_VALUE);
        set_data(T_KEY, T_VALUE);
        assert!(has_data(T_KEY));

        // Now call hello()
        let res_ptr = hello();

        {
            // With AsSlice<u8>
            let res: AsSlice<u8> = AsSlice::from(res_ptr);
            let expected = bytemuck::must_cast_slice(&utf16!("hellw"));
            assert_eq!(res.deref(), expected);
        }

        {
            // With AsSlice<u16>
            // let res: AsSlice<u16> = AsSlice::from(res_ptr);
            let res = AsSlice::<u16>::from(res_ptr);
            assert_eq!(res.deref(), utf16!("hellw"));
        }
    }
}
