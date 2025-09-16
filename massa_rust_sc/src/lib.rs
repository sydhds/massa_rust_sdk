#![no_std]

// Massa blockchain only accept Wasm without the reference-types and multivalue features
// According to this blog post: https://blog.rust-lang.org/2024/09/24/webassembly-targets-change-in-default-target-features/
// this is the default for Rust 1.82+
// So need to compile using Rust nightly (see blog post)
// See also:
// https://github.com/rust-lang/rust/issues/128475
// https://github.com/rust-lang/rust/pull/128511

use lol_alloc::LeakingPageAllocator;
#[global_allocator]
static ALLOCATOR: LeakingPageAllocator = LeakingPageAllocator;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Deref;
use core::ptr::slice_from_raw_parts;
use core::slice;
use bytemuck::Pod;

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

    unsafe {
        v.leak().as_mut_ptr().add(HEADER_SIZE)
    }
}

pub const fn to_as_array<const N: usize>(v: &[u8]) -> [u8; N] {
    let mut dst: [u8; N] = [0u8; N];
    let (a1, a2) = dst.split_at_mut(4);
    a1.copy_from_slice((v.len() as u32).to_le_bytes().as_slice());
    a2.copy_from_slice(v);
    dst
}

#[macro_export]
macro_rules! string_to_as_array {
    ($key:expr) => {{
        const K__: &[u16] = &utf16!($key);
        const K_U8__: &[u8] = bytemuck::must_cast_slice(K__);
        const N__: usize = K_U8__.len();
        to_as_array::<{N__ + 4}>(K_U8__).as_slice()
    }};
}

/*
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    // emit a wasm unreachable instruction if a panic occurs in our code
    core::arch::wasm32::unreachable()
}
*/

pub trait AsMemoryModel {

    const HEADER_SIZE: usize = 4;

    /// Get a pointer to the header
    ///
    /// In the AssemblyScript memory model, the header is just before the data. So in memory, we should have
    /// header|data
    /// header: 4 bytes (size of the data as u32) -> N
    /// data: N bytes
    fn as_ptr_header(&self) -> *const u8;

    /// Get a pointer to the data
    fn as_ptr_data_raw(&self) -> *const u8 {
        unsafe {
            self.as_ptr_header().offset(Self::HEADER_SIZE as isize)
        }
    }

    /// Get a pointer to the data as i32 value
    fn as_ptr_data(&self) -> i32 {
        self.as_ptr_data_raw() as i32
    }
}

impl AsMemoryModel for &[u8] {
    fn as_ptr_header(&self) -> *const u8 {
        {
            // TODO: can we have this checks in the Trait? in Trait::as_ptr_data_raw?
            //       require SuperTrait like: trait AsMemoryModel: AsRef<[u8]> + AsMemoryModel {} ?
            debug_assert!(self.len() >= <&[u8] as AsMemoryModel>::HEADER_SIZE);
            let data_len = u32::from_le_bytes(self[..4].try_into().unwrap());
            debug_assert!(data_len as usize + 4 == self.len());
        }

        self.as_ptr()
    }

}

pub struct AsVec<T>(Vec<T>);

impl<T: Pod> AsVec<T> {

    fn len(&self) -> usize {
        let header_size = <Self as AsMemoryModel>::HEADER_SIZE;
        self.0.len() - header_size
    }

    fn push(&mut self, item: T) {
        // Get current length
        let current_len: u32 = (self.0.len() - 4) as u32;
        // + 1
        let new_len = (current_len + 1).to_le_bytes();
        // Cast to &[u8] so we could update the length (in a generic way)
        let slice: &mut [u8] = bytemuck::cast_slice_mut(self.0.as_mut_slice());
        // Update length
        slice[0] = new_len[0];
        slice[1] = new_len[1];
        slice[2] = new_len[2];
        slice[3] = new_len[3];
        // Push new item
        self.0.push(item);
    }

}

impl FromIterator<u8> for AsVec<u8> {

    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut v = vec![0; 2];
        v.extend(iter);
        let v_len_: u32 = (v.len() - 4) as u32;
        let v_len_bytes = v_len_.to_le_bytes();
        v[0] = v_len_bytes[0];
        v[1] = v_len_bytes[1];
        v[2] = v_len_bytes[2];
        v[3] = v_len_bytes[3];
        Self(v)
    }
}


impl FromIterator<u16> for AsVec<u16> {

    fn from_iter<I: IntoIterator<Item = u16>>(iter: I) -> Self {
        let mut v = vec![0; 2];
        v.extend(iter);
        let v_len_: u32 = (v.len() * 2 - 4) as u32;
        let v_len_bytes = v_len_.to_le_bytes();
        let v_0: [u8; 2] = [v_len_bytes[0], v_len_bytes[1]];
        let v_1: [u8; 2] = [v_len_bytes[2], v_len_bytes[3]];
        v[0] = u16::from_le_bytes(v_0);
        v[1] = u16::from_le_bytes(v_1);
        Self(v)
    }
}

impl<T: Pod> AsMemoryModel for AsVec<T> {
    fn as_ptr_header(&self) -> *const u8 {
        let slice: &[u8] = bytemuck::cast_slice(self.0.as_slice());
        slice.as_ptr()
    }
}

/*
impl AsMemoryModel for AsVec<u8> {
    fn as_ptr_header(&self) -> *const u8 {
        self.0.as_ptr()
    }
}

impl AsMemoryModel for AsVec<u16> {
    fn as_ptr_header(&self) -> *const u8 {
        let slice: &[u8] = bytemuck::cast_slice(self.0.as_slice());
        slice.as_ptr()
    }
}
*/

#[derive(Debug)]
pub struct AsSlice<'a, T>(&'a [T]);

impl<T> Deref for AsSlice<'_, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T: Pod> AsMemoryModel for AsSlice<'a, T> {
    fn as_ptr_header(&self) -> *const u8 {
        let slice: &[u8] = bytemuck::cast_slice(self.0);
        slice.as_ptr()
    }
}

// TODO / FIXME
// In AsMemoryModel trait?
// Better from_ptr_header() & from_ptr_data() ?
// Should be try_from ? if ptr is (can be) NULL?
impl From<*const u8> for AsSlice<'_, u8> {
    fn from(ptr: *const u8) -> Self {

        let res_size = unsafe {
            let res_size_ptr = ptr.offset(-4);
            let slice = slice::from_raw_parts(res_size_ptr, 4);
            u32::from_le_bytes(slice.try_into().unwrap())
        };

        let res = unsafe {
            slice_from_raw_parts(ptr, res_size as usize)
                .as_ref()
                .unwrap()
        };

        Self(res)
    }
}


impl From<*const u8> for AsSlice<'_, u16> {
    fn from(ptr: *const u8) -> Self {

        let res_size = unsafe {
            let res_size_ptr = ptr.offset(-4);
            let slice = slice::from_raw_parts(res_size_ptr, 4);
            u32::from_le_bytes(slice.try_into().unwrap())
        };

        let res = unsafe {
            slice_from_raw_parts(ptr as *const u16, res_size as usize / 2)
                .as_ref()
                .unwrap()
        };

        Self(res)
    }
}

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
    unsafe {
        assembly_script_get_data(key.as_ptr_data())
    }
}

pub fn has_data<T: AsMemoryModel>(key: T) -> bool {
    unsafe {
        assembly_script_has_data(key.as_ptr_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[no_mangle]
    fn __MASSA_RUST_SDK_UNIT_TEST_as_vec_1() {

        let v0 = vec![1u8, 2, 3];
        assert_eq!(v0.len(), 3);

        // FIXME: from_iter assumes the data is already with AS header
        //        maybe add from_iter_as() ? but then it would break collect ?
        let mut av0 = AsVec::from_iter(vec![1u8, 2, 3]);
        // let mut a1 = AsVec::from_iter(vec![1u16, 2, 3]);

        assert!(av0.len() > 3);
        // assert_eq!(a1.len(), 3);

        // a0.push(42);
        // a1.push(42);

        // assert_eq!(a0.len(), 4);
        // assert_eq!(a1.len(), 4);
    }



}