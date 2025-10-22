use alloc::string::String;
use core::ops::Deref;
use crate::{assembly_script_get_call_stack, caller_has_write_access, AsSlice};

/// Return true if the smart contract is currently being deployed
///
/// This function is typically used in the constructor to ensure a one-time deployment and
/// initialization, usually by the creator of the contract.
/// Under the hood, this method verifies that the account calling this function (either the user
/// creating the operation or an upper contract) has write access to the data of the current account
pub fn is_deploying_contract() -> bool {

    // massa-as-sdk code: function isDeployingContract
    // https://github.com/massalabs/massa-as-sdk/blob/main/assembly/std/context.ts

    unsafe {
        if caller_has_write_access() {
            // in isDeployingContract, there are 2 calls to addressStack (one for the caller, one for the callee)
            // Here we try to call it only once
            let call_stack = assembly_script_get_call_stack();
            // Note: assembly_script_get_call_stack return something like:
            //       let s1 = r#"[\"AU1Yvq49utdezr496dHbRj3TMjqsCh2awggjfGraHoddE7XfEkpY\",\"AS12mb3TqNpeers7FRDpYR9XDaFHFxXaG9SuQ1yU778QdjZUa8eQ7\"]"#;
            // but encoded as utf16 string (see as-ffi-bindings - string_ptr.rs file for details)
            let call_stack = AsSlice::<u16>::from(call_stack as *const u8) ;

            // TODO: extract caller & callee as u16 slice & compare
            //       no allocation would be needed

            let s = String::from_utf16_lossy(call_stack.deref());
            let mut s1_split = s[1..s.len() - 1].rsplit(",");

            let callee = s1_split.next().unwrap_or("");
            let caller = s1_split.next().unwrap_or("");

            // in isDeployingContract, checks is done with Address objects,
            // and an Address object is just a wrapper of a string (utf16 string)
            // https://github.com/massalabs/massa-as-sdk/blob/main/assembly/std/address.ts
            callee[2..callee.len() - 2] != caller[2..caller.len() - 2]

        } else {
            false
        }
    }
}