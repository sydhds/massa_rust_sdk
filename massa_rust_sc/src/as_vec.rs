
use alloc::vec;
use alloc::vec::{Drain, Vec};
use core::ops::{
    Range,
    RangeBounds
};
use core::slice::{self, SliceIndex};
// third-party
use bytemuck::Pod;
// internal
use crate::memory::AsMemoryModel;

#[derive(Debug)]
enum UpdateLength {
    Offset(usize),
    Length(usize),
}

/*
pub struct AsVecDrain<'a, T> {
    inner: Drain<'a, T>,
    // len: usize,
}

impl<T> Iterator for AsVecDrain<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
*/

/*
impl<'a, T> FromIterator<T> for AsVecDrain<'a, T> {
    fn from_iter<U: IntoIterator<Item=T>>(iter: U) -> Self {
        todo!()
    }
}
*/

#[derive(Debug)]
pub struct AsVec<T>(Vec<T>);

impl<T: Pod> AsVec<T> {

    pub const fn len(&self) -> usize {
        // let header_size = <Self as AsMemoryModel>::HEADER_SIZE;
        // self.0.len() - header_size
        self.0.len() - (Self::__header_size() / size_of::<T>())
    }

    /*
    fn __byte_len(&self) -> usize {
        self.0.len() * size_of::<T>()
    }
    */

    const fn __header_size() -> usize {
        <Self as AsMemoryModel>::HEADER_SIZE
    }

    fn __update_as_header(&mut self, l: UpdateLength) {
        // current length + 1
        let new_len =  match l {
            UpdateLength::Offset(offset) => {
                self.len() + offset
            },
            UpdateLength::Length(nl) => {
                nl
            },
        }.to_le_bytes();

        // let msg = format!("len: {:?} - offset: {:?}", self.len(), l);
        // generate_event( msg.encode_utf16().collect::<AsVec<u16>>());
        // let msg = format!("new_len: {:?}", new_len);
        // generate_event( msg.encode_utf16().collect::<AsVec<u16>>());

        // Cast to &[u8] so we could update the length (in a generic way)
        let slice: &mut [u8] = bytemuck::cast_slice_mut(self.0.as_mut_slice());
        // Update length
        slice[0] = new_len[0];
        slice[1] = new_len[1];
        slice[2] = new_len[2];
        slice[3] = new_len[3];
    }

    fn __as_raw_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    pub fn append(&mut self, other: &mut Self) {
        self.__update_as_header(UpdateLength::Offset(other.len() * size_of::<T>()));
        self.0.extend(&other.0[4..]);
        other.clear();
    }

    /*
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    */

    pub fn clear(&mut self) {
        self.__update_as_header(UpdateLength::Length(0));
        self.0.drain(4..);
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        self.__update_as_header(UpdateLength::Offset(other.len() * size_of::<T>()));
        self.0.extend_from_slice(other);
    }

    pub fn insert(&mut self, index: usize, element: T) {
        self.__update_as_header(UpdateLength::Offset(size_of::<T>()));
        self.0.insert((Self::__header_size() / size_of::<T>()) + index, element);
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, item: T) {
        self.__update_as_header(UpdateLength::Offset(size_of::<T>()));
        // Push new item
        self.0.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {

        let inner_len = self.len();
        if inner_len == 0 {
            return None;
        }
        let res = self.0.pop();
        if res.is_some() {
            self.__update_as_header(UpdateLength::Length(inner_len - size_of::<T>()));
        }
        res
    }

    pub fn remove(&mut self, index: usize) -> T {

        let inner_len = self.len() * size_of::<T>();
        let res = self.0.remove(index + (Self::__header_size() / size_of::<T>()));
        self.__update_as_header(UpdateLength::Length(inner_len - size_of::<T>()));
        res
    }

}


impl AsVec<u8> {

    pub fn new() -> Self {
        let inner: Vec<u8> = vec![0, Self::HEADER_SIZE as u8];
        Self(
           inner
        )
    }

    /*
    pub fn drain<'a, R>(&'a mut self, range: R) -> Drain<'a, u8> where R: RangeBounds<usize> {

        let self_0 = &mut self.0;
        let self_0_1 = &mut *self_0;
        let inner = core::mem::take(self_0);
        let (ptr, len, cap) = inner.into_raw_parts();

        let mut rebuilt = unsafe {
            // We can now make changes to the components, such as
            // transmuting the raw pointer to a compatible type.
            // let ptr = ptr as *mut u8;
            Vec::from_raw_parts(ptr.offset(Self::HEADER_SIZE as isize), len - Self::HEADER_SIZE, cap - Self::HEADER_SIZE)
        };

        unsafe {
            *self_0 = rebuilt;
            let res = self_0.drain(range);

            // self.0 = Vec::from_raw_parts(self.0.as_mut_ptr().offset(Self::HEADER_SIZE as isize), len, cap);
            // res
            *self_0_1 = Vec::from_raw_parts(self_0_1.as_mut_ptr().offset(Self::HEADER_SIZE as isize), len, cap);
                res
        }
    }
    */
}

impl AsVec<u16> {
    pub fn new() -> Self {
        let inner: Vec<u16> = vec![0, (Self::HEADER_SIZE / size_of::<u16>()) as u16];
        Self(
            inner
        )
    }
}

impl<T: Pod + PartialEq> AsVec<T> {

    pub fn dedup(&mut self) {
        todo!()
    }
}

impl FromIterator<u8> for AsVec<u8> {

    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut v = vec![0; 4];
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

#[cfg(test)]
mod tests {
    use alloc::format;
    use crate::{generate_event, get_data, set_data, AsSlice};
    use super::*;

    #[test]
    #[no_mangle]
    fn __MASSA_RUST_SDK_UNIT_TEST_as_vec_append() {

        let mut v0 = AsVec::from_iter(vec![1u8, 2, 3]);
        let mut v1 = AsVec::from_iter(vec![255u8]);
        let expected_len = v0.len() + v1.len();
        v1.append(&mut v0);
        assert_eq!(v1.len(), expected_len);
        assert_eq!(v0.len(), 0);
    }

    #[test]
    #[no_mangle]
    fn __MASSA_RUST_SDK_UNIT_TEST_as_vec_push() {

        let v0 = vec![1u8, 2, 3];
        assert_eq!(v0.len(), 3);
        let mut av0 = AsVec::from_iter(vec![1u8, 2, 3]);
        assert_eq!(av0.len(), 3);

        av0.push(42);
        assert_eq!(av0.len(), 4);

        let mut av1 = AsVec::from_iter(vec![1u16, 2, 3]);
        assert_eq!(av1.len(), 3);

        av1.push(42);
        av1.push(42);
        assert_eq!(av1.len(), 5);

        let mut av_0: AsVec<u16> = AsVec::from_iter(vec![]);
        assert_eq!(av_0.len(), 0);
        let mut av_0: AsVec<u8> = AsVec::from_iter(vec![]);
        assert_eq!(av_0.len(), 0);
    }

    #[test]
    #[no_mangle]
    fn __MASSA_RUST_SDK_UNIT_TEST_as_vec_insert() {
        let mut v = AsVec::from_iter(vec![1u8, 255]);
        assert_eq!(v.len(), 2);
        v.insert(1, 42);
        assert_eq!(v.len(), 3);
        assert_eq!(&v.__as_raw_slice()[4..], &[1, 42, 255]);
        v.insert(3, 41);
        assert_eq!(v.len(), 4);
        assert_eq!(&v.__as_raw_slice()[4..], &[1, 42, 255, 41]);
        v.insert(0, 40);
        assert_eq!(v.len(), 5);
        assert_eq!(&v.__as_raw_slice()[4..], &[40, 1, 42, 255, 41]);
    }

    #[test]
    #[no_mangle]
    fn __MASSA_RUST_SDK_UNIT_TEST_as_vec_pop() {
        let mut v = AsVec::from_iter(vec![1u8, 255]);
        assert_eq!(v.len(), 2);
        assert_eq!(v.pop(), Some(255));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);
        assert_eq!(v.len(), 0);
    }

    #[test]
    #[no_mangle]
    fn __MASSA_RUST_SDK_UNIT_TEST_as_vec_clear() {

        let mut v = AsVec::from_iter(vec![1u8, 2, 3]);

        assert_eq!(v.len(), 3);
        // let msg = format!("v len: {}", v.len());
        // generate_event( msg.encode_utf16().collect::<AsVec<u16>>());
        // let msg = format!("v: {:?}", v.__as_raw_slice());
        // generate_event( msg.encode_utf16().collect::<AsVec<u16>>());
        v.clear();
        // let msg = format!("v: {:?}", v.__as_raw_slice());
        // generate_event( msg.encode_utf16().collect::<AsVec<u16>>());
        assert_eq!(v.len(), 0);
        assert!(v.is_empty());

        v.push(42);
        assert_eq!(v.len(), 1);
    }

    #[test]
    #[no_mangle]
    fn __MASSA_RUST_SDK_UNIT_TEST_as_vec_remove() {

        {
            let mut v = AsVec::from_iter(vec![1u8, 2, 3]);
            assert_eq!(v.len(), 3);

            let rm_1 = v.remove(1);

            assert_eq!(v.len(), 2);
            // let msg = format!("v: {:?}", v.__as_raw_slice());
            // generate_event(msg.encode_utf16().collect::<AsVec<u16>>());
            assert_eq!(v.__as_raw_slice(), &[2, 0, 0, 0, 1, 3]);
        }

        // AsVec<u16>
        {
            let mut v = AsVec::from_iter(vec![1u16, 2, 3]);

            let v_s = v.__as_raw_slice();
            let v_s_2: &[u8] = bytemuck::cast_slice(v_s);
            let msg = format!("v: {:?}", v_s_2);
            generate_event(msg.encode_utf16().collect::<AsVec<u16>>());
            assert_eq!(v.len(), 3);

            let rm_1 = v.remove(1);
            assert_eq!(v.len(), 2);

            let v_s = v.__as_raw_slice();
            let v_s_2: &[u8] = bytemuck::cast_slice(v_s);
            let msg = format!("v: {:?}", v_s_2);
            generate_event(msg.encode_utf16().collect::<AsVec<u16>>());

            assert_eq!(v_s_2, &[4, 0, 0, 0, 1, 0, 3, 0]);
        }
    }

    /*
    #[test]
    #[no_mangle]
    fn __MASSA_RUST_SDK_UNIT_TEST_as_vec_drain() {

        let mut v = AsVec::from_iter(vec![1u8, 2, 251, 255]);

        let msg = format!("v: {:?}", v.__as_raw_slice());
        generate_event(msg.encode_utf16().collect::<AsVec<u16>>());

        let u: Vec<_> = v
            .drain(1..)
            .collect();

        let msg = format!("v: {:?}", v.__as_raw_slice());
        generate_event(msg.encode_utf16().collect::<AsVec<u16>>());
        // assert_eq!(v.len(), 1);
    }
    */

    /*
    #[test]
    #[no_mangle]
    fn __MASSA_RUST_SDK_UNIT_TEST_as_vec_storage() {

        let mut k1 = AsVec::from_iter(vec![250u16]);
        let mut k1_2 = AsVec::from_iter(vec![250u16]);
        let mut k2 = AsVec::from_iter(vec![150u16]);
        let mut k2_2 = AsVec::from_iter(vec![150u16]);
        let mut v1 = AsVec::from_iter(vec![1u16, 2, 3]);
        let mut v2 = AsVec::from_iter(vec![41u16, 42, 43]);

        // assert_eq!(v.len(), 3);
        // let rm_1 = v.remove(1);

        set_data(k1, v1);
        set_data(k2, v2);

        let v1_res_ = get_data(k1_2) as *const u8;

        /*
        let v1_res: AsSlice<u16> = AsSlice::from(v1_res_);
        let msg = format!("v: {:?}", v1_res.as_ref());
        generate_event(msg.encode_utf16().collect::<AsVec<u16>>());
        */

        let v1_res: AsSlice<u8> = AsSlice::from(v1_res_);
        let msg = format!("v: {:?}", v1_res.as_ref());
        generate_event(msg.encode_utf16().collect::<AsVec<u16>>());

        // let v2_res = get_data(k2_2);
        //
        // let msg = format!("v: {:?}", v2_res.__as_raw_slice());
        // generate_event(msg.encode_utf16().collect::<AsVec<u16>>());
    }
    */
}
