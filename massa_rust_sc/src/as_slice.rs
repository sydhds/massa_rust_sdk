use core::ptr::slice_from_raw_parts;
use core::slice;
use core::ops::Deref;
// third-party
use bytemuck::Pod;
// internal
use crate::memory::AsMemoryModel;

pub struct AsArray<T, const N: usize>([T; N]);

impl<T, const N: usize> AsArray<T, N> {
    pub const fn as_slice(&self) -> AsSlice<'_, T> {
        AsSlice(&self.0)
    }
}


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

pub const fn to_as_array<const N: usize>(v: &[u8]) -> AsArray<u8, N> {
    let mut dst: [u8; N] = [0u8; N];
    let (a1, a2) = dst.split_at_mut(4);
    a1.copy_from_slice((v.len() as u32).to_le_bytes().as_slice());
    a2.copy_from_slice(v);
    AsArray(dst)
}

#[macro_export]
macro_rules! to_as_slice {
    ($key:expr) => {{
        const K__: &[u16] = &utf16!($key);
        const K_U8__: &[u8] = bytemuck::must_cast_slice(K__);
        const N__: usize = K_U8__.len();
        to_as_array::<{N__ + 4}>(K_U8__).as_slice()
    }};
}
