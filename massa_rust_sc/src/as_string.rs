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
