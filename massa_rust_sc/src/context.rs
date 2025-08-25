/*
use crate::{caller_has_write_access, get_call_stack};

pub fn is_deploying_contract() -> bool {
    unsafe {
        if caller_has_write_access() {
            let call_stack = get_call_stack();
            // FIXME
            false
        } else {
            false
        }
    }
}
*/