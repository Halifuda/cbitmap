//! Implementations of logic operations like [`BitAnd`] and [`BitOrAssign`] 
//! for `Bitmap`. 
//! 
//! Also including [`Deref`] of `BitRef` and `BitRefMut`.

use super::{*, refs::*};
use core::ops::{BitAnd, BitAndAssign, BitOrAssign, Deref};

impl<'map, const BYTES: usize> Deref for BitRef<'map, BYTES> {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'map, const BYTES: usize> Deref for BitRefMut<'map, BYTES> {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<const BYTES: usize, const N: usize> BitAnd<[u8; N]> for &Bitmap<BYTES> {
    type Output = [u8; N];

    /// AND the given bitmap (in `ref`) to an array of [`u8`] values.
    ///
    /// # Generics
    /// * `BYTES`: the byte length of the bitmap.
    /// * `N`: the length of the [`u8`] array.
    ///
    /// # Examples
    /// A `&` can AND the bitmap with a fixed-length [`u8`] array, the results
    /// will be store in a new array:
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = Bitmap::<2>::from([255u8; 2]);
    /// let arr = [1u8; 2];
    /// // NOTE: the bitmap shoule be in ref, and should be in left.
    /// assert_eq!(&map & arr, [1u8; 2]);
    /// ```
    /// There are also aliases for integer types:
    /// ```
    /// use cbitmap::bitmap::*;
    /// 
    /// let mut map = Bitmap::<1>::from(255u8);
    /// let arr = 1u8;
    /// assert_eq!(&map & arr, 1u8);
    /// ```
    /// # See
    /// About the asymmetry between `BYTES` and `N`, see
    /// `bitmap::Bitmap<BYTES>::bitand_assign`.
    ///
    /// In general, if `N > BYTES`, the
    /// returned array will have `（N - BYTES） * 8` leading zero flags.
    fn bitand(self, rhs: [u8; N]) -> Self::Output {
        let size = N.min(BYTES);
        let mut arr = rhs.clone();
        for i in 0..size {
            let byte = self.__copy_u8(i);
            arr[i] &= byte;
        }
        if N > BYTES {
            for i in size..N {
                arr[i] = 0;
            }
        }
        arr
    }
}

impl<const BYTES: usize, const N: usize> BitAndAssign<[u8; N]> for Bitmap<BYTES> {
    /// AND the given bitmap with an array of [`u8`] values.
    ///
    /// # Generics
    /// * `BYTES`: the byte length of the bitmap.
    /// * `N`: the length of the [`u8`] array.
    ///
    /// # Examples
    /// A simple example:
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = Bitmap::<1>::from([0b_11111111_u8; 1]);
    /// map &= [0b_11111110_u8; 1];
    /// assert_eq!(map.test(0), false);
    /// ```
    /// 
    /// There are also aliases for integer types:
    /// ```
    /// use cbitmap::bitmap::*;
    /// 
    /// let mut map = Bitmap::<1>::from(255u8);
    /// map &= 0b_11111110u8;
    /// assert_eq!(map.test(0), false);
    /// ```
    ///
    /// It is also noteworthy that, if the bitmap is longer than the array,
    /// then the rest of bitmap will be set to all-zero.
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map: Bitmap::<2> = [255u8; 2].into();
    /// map &= 255u8;
    ///
    /// assert_eq!(&map.range_to_string(8, 16).unwrap(), "00000000");
    /// ```
    fn bitand_assign(&mut self, rhs: [u8; N]) {
        let size = N.min(BYTES);
        for i in 0..size {
            __byte_and_u8(self.__get_mut_u8(i), rhs[i]);
        }
        if BYTES > N {
            for i in size..BYTES {
                *self.__get_mut_u8(i) = 0;
            }
        }
    }
}

impl<const BYTES: usize, const N: usize> BitOrAssign<[u8; N]> for Bitmap<BYTES> {
    /// OR the given bitmap with an array of [`u8`] values.
    ///
    /// # Generics
    /// * `BYTES`: the byte length of the bitmap.
    /// * `N`: the length of the [`u8`] array.
    ///
    /// # Examples
    /// A simple example:
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = Bitmap::<1>::from([0u8; 1]);
    /// map |= [0b_00000001_u8; 1];
    /// assert_eq!(map.test(0), true);
    /// ```
    /// 
    /// There are also aliases for integer types:
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = Bitmap::<1>::from(0u8);
    /// map |= 0b_00000001_u8;
    /// assert_eq!(map.test(0), true);
    /// ```
    fn bitor_assign(&mut self, rhs: [u8; N]) {
        let size = N.min(BYTES);
        for i in 0..size {
            __byte_or_u8(self.__get_mut_u8(i), rhs[i]);
        }
    }
}

macro_rules! impl_bitand {
    ($t:ty) => {
        impl<const BYTES: usize> BitAnd<$t> for &Bitmap<BYTES> {
            type Output = $t;
            fn bitand(self, rhs: $t) -> Self::Output {
                const SIZE: usize = core::mem::size_of::<$t>();
                let arr:[u8; SIZE] = unsafe { core::mem::transmute(rhs) };
                let res = self & arr;
                unsafe { core::mem::transmute(res) }
            }
        }
    };
}

macro_rules! impl_bitand_assign {
    ($t:ty) => {
        impl<const BYTES: usize> BitAndAssign<$t> for Bitmap<BYTES> {
            fn bitand_assign(&mut self, rhs: $t) {
                const SIZE: usize = core::mem::size_of::<$t>();
                let arr:[u8; SIZE] = unsafe { core::mem::transmute(rhs) };
                *self &= arr
            }
        }
    };
}

macro_rules! impl_bitor_assign {
    ($t:ty) => {
        impl<const BYTES: usize> BitOrAssign<$t> for Bitmap<BYTES> {
            fn bitor_assign(&mut self, rhs: $t) {
                const SIZE: usize = core::mem::size_of::<$t>();
                let arr:[u8; SIZE] = unsafe { core::mem::transmute(rhs) };
                *self |= arr
            }
        }
    };
}

impl_bitand!(u8);
impl_bitand!(i8);
impl_bitand!(char);
impl_bitand!(u16);
impl_bitand!(i16);
impl_bitand!(u32);
impl_bitand!(i32);
impl_bitand!(u64);
impl_bitand!(i64);
impl_bitand!(u128);
impl_bitand!(i128);
impl_bitand!(usize);
impl_bitand!(isize);

impl_bitand_assign!(u8);
impl_bitand_assign!(i8);
impl_bitand_assign!(char);
impl_bitand_assign!(u16);
impl_bitand_assign!(i16);
impl_bitand_assign!(u32);
impl_bitand_assign!(i32);
impl_bitand_assign!(u64);
impl_bitand_assign!(i64);
impl_bitand_assign!(u128);
impl_bitand_assign!(i128);
impl_bitand_assign!(usize);
impl_bitand_assign!(isize);

impl_bitor_assign!(u8);
impl_bitor_assign!(i8);
impl_bitor_assign!(char);
impl_bitor_assign!(u16);
impl_bitor_assign!(i16);
impl_bitor_assign!(u32);
impl_bitor_assign!(i32);
impl_bitor_assign!(u64);
impl_bitor_assign!(i64);
impl_bitor_assign!(u128);
impl_bitor_assign!(i128);
impl_bitor_assign!(usize);
impl_bitor_assign!(isize);
