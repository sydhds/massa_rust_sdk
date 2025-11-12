use crate::{assembly_script_get_call_stack, caller_has_write_access, AsSlice};

// https://doc.rust-lang.org/std/ascii/enum.Char.html#variant.Comma
const COMMA_CHAR: u16 = 44; // == ','

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
            let call_stack = AsSlice::<u16>::from(call_stack as *const u8);

            let mut call_stack_split = call_stack
                .as_ref()
                [1..call_stack.len() - 1] // remove '[' && ']' characters
                .rsplitn(2, |c| *c == COMMA_CHAR);

            // Unwrap safe: assume call_stack string is formatted as expected (see the above comment)
            let callee = call_stack_split.next().unwrap();
            // Note: call stack len can be < 2. In this case, in massa-as-sdk code, callee is returned as caller
            let caller = call_stack_split.next().unwrap_or(callee);

            // in isDeployingContract, checks is done with Address objects,
            // and an Address object is just a wrapper of a string (utf16 string)
            // https://github.com/massalabs/massa-as-sdk/blob/main/assembly/std/address.ts
            // Note: slicing with 2..len - 2 is to remove the following characters: \"
            callee[2..callee.len() - 2] != caller[2..caller.len() - 2]

        } else {
            false
        }
    }
}


pub fn get_call_stack<'a>() -> AsSlice<'a, u16> {
    unsafe {
        let call_stack = assembly_script_get_call_stack();
        // Note: assembly_script_get_call_stack return something like:
        //       let s1 = r#"[\"AU1Yvq49utdezr496dHbRj3TMjqsCh2awggjfGraHoddE7XfEkpY\",\"AS12mb3TqNpeers7FRDpYR9XDaFHFxXaG9SuQ1yU778QdjZUa8eQ7\"]"#;
        // but encoded as utf16 string (see as-ffi-bindings - string_ptr.rs file for details)
        let call_stack = AsSlice::<u16>::from(call_stack as *const u8);
        call_stack
    }
}

/// Returns the address of the currently executing smart contract.
///
/// The "callee" refers to the contract that is currently being executed.
pub fn callee<'a>(call_stack: &'a AsSlice<'a, u16>) -> &'a [u16] {
    let mut call_stack_split = call_stack
        .as_ref()
        [1..call_stack.len() - 1] // remove '[' && ']' characters
        .rsplitn(1, |c| *c == COMMA_CHAR);
    // Unwrap safe: assume call_stack string is formatted as expected (see the above comment)
    let callee = call_stack_split.next().unwrap();
    &callee[2..callee.len() - 2]
}

/// Returns the `address` of the `caller` of the currently executing smart contract.
///
/// The caller is the person or the smart contract that directly called
/// the pending function.
pub fn caller<'a>(call_stack: &'a AsSlice<'a, u16>) -> &'a [u16] {
    let mut call_stack_split = call_stack
        .as_ref()
        [1..call_stack.len() - 1] // remove '[' && ']' characters
        .rsplitn(1, |c| *c == COMMA_CHAR);
    let callee = call_stack_split.next().unwrap();
    // Note: call stack len can be < 2. In this case, in massa-as-sdk code, callee is returned as caller
    let caller = call_stack_split.next();

    if caller.is_none() {
        &callee[2..callee.len() - 2]
    } else {
        let caller = caller.unwrap();
        &caller[2..caller.len() - 2]
    }
}

/// Returns the address of the initial transaction creator (originator).
pub fn transaction_creator<'a>(call_stack: &'a AsSlice<'a, u16>) -> &'a [u16] {
    let mut call_stack_split = call_stack
        .as_ref()
        [1..call_stack.len() - 1] // remove '[' && ']' characters
        .splitn(1, |c| *c == COMMA_CHAR);
    let creator = call_stack_split.next().unwrap();
    &creator[2..creator.len() - 2]
}

