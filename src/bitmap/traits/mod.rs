//! Here we implement several methods for [`Bitmap`], which should be overrided.

use super::*;

// Overrided methods

/// Fill the first several bytes (8*bits) of a bitmap.
pub trait FillPrefix<T: Sized> {
    fn fill_prefix(&mut self, value:T) -> &mut Self;
}

impl<const BYTES: usize, const N: usize> FillPrefix<[u8; N]> for Bitmap<BYTES> {
    /// Fill the first N bytes (N*8 bits) of a bitmap with given byte array.
    /// 
    /// # Return
    /// `&mut self`, allowing a call chain.
    /// 
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    /// 
    /// let mut map = newmap!(;16);
    /// map.fill_prefix([0b_1010u8; 1]);
    /// assert_eq!(map.test(1), true);
    /// assert_eq!(map.test(3), true);
    /// ```
    /// Here are some aliases:
    /// ```
    /// use cbitmap::bitmap::*;
    /// 
    /// let mut map = newmap!(;128);
    /// // aliases
    /// map.fill_prefix(1u8 << 7);
    /// assert_eq!(map.test(7), true);
    /// map.fill_prefix(1u16 << 15);
    /// assert_eq!(map.test(15), true);
    /// // ... 
    /// map.fill_prefix(1i128 << 100);
    /// assert_eq!(map.test(100), true);
    /// ```
    fn fill_prefix(&mut self, mut value: [u8; N]) -> &mut Self {
        __copy_bytes_to(&mut self.bits, &mut value);
        self
    }
}

// Alias impls
// FillPrefix

macro_rules! impl_fill_prefix {
    ($t:ty) => {
        impl<const BYTES: usize> FillPrefix<$t> for Bitmap<BYTES> {
            fn fill_prefix(&mut self, mut value: $t) -> &mut Self {
                const SIZE: usize = core::mem::size_of::<$t>();
                unsafe {
                    let ptr = (&mut value as *mut $t).cast::<[u8; SIZE]>();
                    __copy_bytes_to(&mut self.bits, &mut *ptr);
                };
                self
            }
        }
    };
}

impl_fill_prefix!(u8);
impl_fill_prefix!(i8);
impl_fill_prefix!(char);
impl_fill_prefix!(u16);
impl_fill_prefix!(i16);
impl_fill_prefix!(u32);
impl_fill_prefix!(i32);
impl_fill_prefix!(u64);
impl_fill_prefix!(i64);
impl_fill_prefix!(u128);
impl_fill_prefix!(i128);
impl_fill_prefix!(usize);
impl_fill_prefix!(isize);