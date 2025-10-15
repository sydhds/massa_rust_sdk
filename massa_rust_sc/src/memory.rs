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
        unsafe { self.as_ptr_header().add(Self::HEADER_SIZE) }
    }

    /// Get a pointer to the data as i32 value
    fn as_ptr_data(&self) -> i32 {
        self.as_ptr_data_raw() as i32
    }
}
