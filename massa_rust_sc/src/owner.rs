use utf16_lit::utf16;
use crate::{
    to_as_array,
    string_to_as_array
};

pub const OWNER_KEY: &[u8] = string_to_as_array!("OWNER");
pub const CHANGE_OWNER_EVENT_NAME: &[u8] = string_to_as_array!("CHANGE_OWNER");
