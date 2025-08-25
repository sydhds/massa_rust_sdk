use utf16_lit::utf16;
use crate::{to_as_array, to_as_slice, AsSlice};

pub const OWNER_KEY: AsSlice<u8> = to_as_slice!("OWNER");
pub const CHANGE_OWNER_EVENT_NAME: AsSlice<u8> = to_as_slice!("CHANGE_OWNER");
