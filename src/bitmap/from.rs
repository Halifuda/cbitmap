//! Implementations of [`From`] and [`Into`] for `Bitmap`, `BitRef` and `BitRefMut`.
//!
//! Allowing converting between them and a set of native types like [`bool`].

use crate::bitmap::*;

impl<const BYTES: usize> Default for Bitmap<BYTES> {
    /// Default bitmap. All the bits are set to 0.
    fn default() -> Self {
        Bitmap::<BYTES>::new()
    }
}

// Into

impl<const BYTES: usize> Into<[u8; BYTES]> for Bitmap<BYTES> {
    /// Give the inner array of bitmap.
    ///
    /// # See
    /// [`Bitmap`].
    fn into(self) -> [u8; BYTES] {
        match BYTES == 0 {
            true => [0; BYTES],
            false => self.bits.unwrap(),
        }
    }
}

impl<'map, const BYTES: usize> Into<bool> for BitRef<'map, BYTES> {
    /// Give the value of the referenced bit.
    ///
    /// # See
    /// [`BitRef`].
    fn into(self) -> bool {
        self.value
    }
}

impl<'map, const BYTES: usize> Into<bool> for BitRefMut<'map, BYTES> {
    /// Give the value of the reverence bit.
    /// # See
    /// [`BitRefMut`].
    fn into(self) -> bool {
        self.value
    }
}

// Tool for From

fn __copy_bytes<const N: usize, const M: usize>(src: [u8; M]) -> [u8; N] {
    let mut dst = [0u8; N];
    unsafe {
        match N > M {
            true => {
                let dstptr = dst.as_mut_ptr().cast::<[u8; M]>();
                core::ptr::write(dstptr, src);
            }
            false => {
                let srcptr = src.as_ptr().cast::<[u8; N]>();
                let dstptr = &mut dst as *mut [u8; N];
                *dstptr = *srcptr;
            }
        }
    };
    dst
}

// From

impl<const BYTES: usize, const N: usize> From<[u8; N]> for Bitmap<BYTES> {
    /// Convert a array `[u8; N]` into `Bitmap<BYTES>`.
    ///
    /// The generics `N` and `BYTES` has not to be equal.
    /// If `N < BYTES`, the bitmap will have `BYTES - N`
    /// bytes of leading zero.
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    ///
    /// let map = Bitmap::<2>::from([0u8; 2]);
    /// ```
    fn from(value: [u8; N]) -> Self {
        match BYTES == 0 {
            true => Bitmap::<BYTES>::new(),
            false => Bitmap {
                bits: Some(__copy_bytes(value)),
            },
        }
    }
}

macro_rules! impl_from {
    ($t:ty) => {
        impl<const BYTES: usize> From<$t> for Bitmap<BYTES> {
            fn from(value: $t) -> Self {
                let arr: [u8; core::mem::size_of::<$t>()] = unsafe { core::mem::transmute(value) };
                Bitmap::<BYTES>::from(arr)
            }
        }
    };
}

impl_from!(u8);
impl_from!(i8);
impl_from!(char);
impl_from!(u16);
impl_from!(i16);
impl_from!(u32);
impl_from!(i32);
impl_from!(u64);
impl_from!(i64);
impl_from!(u128);
impl_from!(i128);
impl_from!(usize);
impl_from!(isize);
