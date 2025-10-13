use alloc::vec;
use alloc::vec::Vec;
// third-party
use bytemuck::Pod;
// internal
use crate::memory::AsMemoryModel;

#[derive(Debug)]
enum UpdateLength {
    Offset(usize),
    Length(usize),
}

#[derive(Debug)]
pub struct AsVec<T>(Vec<T>);

impl<T: Pod> AsVec<T> {

    pub const fn len(&self) -> usize {
        // let header_size = <Self as AsMemoryModel>::HEADER_SIZE;
        // self.0.len() - header_size
        self.0.len() - (Self::__header_size() / size_of::<T>())
    }

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
}
