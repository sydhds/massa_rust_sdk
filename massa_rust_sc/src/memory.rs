pub trait AsMemoryModel {

    const HEADER_SIZE: usize = 4;

    /// Get a pointer to the header
    ///
    /// In the AssemblyScript memory model, the header is just before the data. So in memory, we should have
    /// header|data
    /// header: 4 bytes (size of the data as u32) -> N
    /// data: N bytes
    /// See https://www.assemblyscript.org/runtime.html#memory-layout
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

/*
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
*/