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

impl FromIterator<u16> for AsVec<u16> {
    fn from_iter<I: IntoIterator<Item = u16>>(iter: I) -> Self {

        let mut v = vec![0; 2];
        v.extend(iter);
        let v_len_: u32 = (v.len() * 2 - 2) as u32;
        let v_len_bytes = v_len_.to_le_bytes();
        let v_0: [u8; 2] = [v_len_bytes[0], v_len_bytes[1]];
        let v_1: [u8; 2] = [v_len_bytes[2], v_len_bytes[3]];
        v[0] = u16::from_le_bytes(v_0);
        v[1] = u16::from_le_bytes(v_1);
        Self(v)
    }
}

impl AsMemoryModel for AsVec<u16> {
    fn as_ptr_header(&self) -> *const u8 {
        let slice: &[u8] = bytemuck::cast_slice(self.0.as_slice());
        slice.as_ptr()
    }
}

pub fn generate_event<T: AsMemoryModel>(event: T) {
    unsafe {
        assembly_script_generate_event(event.as_ptr_data());
    }
}