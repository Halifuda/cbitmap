//! Implementations of logic operations like [`BitAnd`] and [`BitOrAssign`] 
//! for `Bitmap`. 
//! 
//! Also including [`Deref`] of `BitRef` and `BitRefMut`.

use crate::bitmap::*;
use core::ops::{BitAnd, BitAndAssign, BitOrAssign, Deref};

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
    /// use cbitmap::bitmap::Bitmap;
    ///
    /// let mut map: Bitmap::<2> = [255u8; 2].into();
    /// let arr = [1u8; 2];
    /// // NOTE: the bitmap shoule be in ref, and should be in left.
    /// assert_eq!(&map & arr, [1u8; 2]);
    /// ```
    /// 
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
    /// use cbitmap::bitmap::Bitmap;
    ///
    /// let mut map: Bitmap::<1> = [0b_11111111_u8; 1].into();
    /// map &= [0b_11111110_u8; 1];
    /// assert_eq!(map.get_bool(0), false);
    /// ```
    /// 
    /// The **override** of AND a bitmap with fixed-size-unsigned integer is
    /// provided. 
    /// For example, [`u8`], [`u16`] or even [`u128`] can be AND to
    /// a bitmap.
    ///
    /// It is also noteworthy that, if the bitmap is longer than the array,
    /// then the rest of bitmap will be set to all-zero.
    /// ```
    /// use cbitmap::bitmap::Bitmap;
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
    /// use cbitmap::bitmap::Bitmap;
    ///
    /// let mut map: Bitmap::<1> = [0u8; 1].into();
    /// map |= [0b_00000001_u8; 1];
    /// assert_eq!(map.get_bool(0), true);
    /// ```
    /// 
    /// The **override** of OR a bitmap with fixed-size-unsigned integer is
    /// provided. 
    /// For example, [`u8`], [`u16`] or even [`u128`] can be AND to
    /// a bitmap.
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    ///
    /// let mut map: Bitmap::<1> = [0u8; 1].into();
    /// map |= 0b_00000001_u8;
    /// assert_eq!(map.get_bool(0), true);
    /// ```
    fn bitor_assign(&mut self, rhs: [u8; N]) {
        let size = N.min(BYTES);
        for i in 0..size {
            __byte_or_u8(self.__get_mut_u8(i), rhs[i]);
        }
    }
}

impl<const BYTES: usize> BitAnd<u8> for &Bitmap<BYTES> {
    type Output = u8;
    fn bitand(self, rhs: u8) -> Self::Output {
        let arr: [u8; 1] = unsafe { core::mem::transmute(rhs) };
        let arr = self & arr;
        unsafe { core::mem::transmute(arr) }
    }
}

impl<const BYTES: usize> BitAndAssign<u8> for Bitmap<BYTES> {
    fn bitand_assign(&mut self, rhs: u8) {
        let arr: [u8; 1] = unsafe { core::mem::transmute(rhs) };
        *self &= arr;
    }
}

impl<const BYTES: usize> BitOrAssign<u8> for Bitmap<BYTES> {
    fn bitor_assign(&mut self, rhs: u8) {
        let arr: [u8; 1] = unsafe { core::mem::transmute(rhs) };
        *self |= arr;
    }
}

impl<const BYTES: usize> BitAnd<u16> for &Bitmap<BYTES> {
    type Output = u16;
    fn bitand(self, rhs: u16) -> Self::Output {
        let arr: [u8; 2] = unsafe { core::mem::transmute(rhs) };
        let arr = self & arr;
        unsafe { core::mem::transmute(arr) }
    }
}

impl<const BYTES: usize> BitAndAssign<u16> for Bitmap<BYTES> {
    fn bitand_assign(&mut self, rhs: u16) {
        let arr: [u8; 2] = unsafe { core::mem::transmute(rhs) };
        *self &= arr;
    }
}

impl<const BYTES: usize> BitOrAssign<u16> for Bitmap<BYTES> {
    fn bitor_assign(&mut self, rhs: u16) {
        let arr: [u8; 2] = unsafe { core::mem::transmute(rhs) };
        *self |= arr;
    }
}

impl<const BYTES: usize> BitAnd<u32> for &Bitmap<BYTES> {
    type Output = u32;
    fn bitand(self, rhs: u32) -> Self::Output {
        let arr: [u8; 4] = unsafe { core::mem::transmute(rhs) };
        let arr = self & arr;
        unsafe { core::mem::transmute(arr) }
    }
}

impl<const BYTES: usize> BitAndAssign<u32> for Bitmap<BYTES> {
    fn bitand_assign(&mut self, rhs: u32) {
        let arr: [u8; 4] = unsafe { core::mem::transmute(rhs) };
        *self &= arr;
    }
}

impl<const BYTES: usize> BitOrAssign<u32> for Bitmap<BYTES> {
    fn bitor_assign(&mut self, rhs: u32) {
        let arr: [u8; 4] = unsafe { core::mem::transmute(rhs) };
        *self |= arr;
    }
}

impl<const BYTES: usize> BitAnd<u64> for &Bitmap<BYTES> {
    type Output = u64;
    fn bitand(self, rhs: u64) -> Self::Output {
        let arr: [u8; 8] = unsafe { core::mem::transmute(rhs) };
        let arr = self & arr;
        unsafe { core::mem::transmute(arr) }
    }
}

impl<const BYTES: usize> BitAndAssign<u64> for Bitmap<BYTES> {
    fn bitand_assign(&mut self, rhs: u64) {
        let arr: [u8; 8] = unsafe { core::mem::transmute(rhs) };
        *self &= arr;
    }
}

impl<const BYTES: usize> BitOrAssign<u64> for Bitmap<BYTES> {
    fn bitor_assign(&mut self, rhs: u64) {
        let arr: [u8; 8] = unsafe { core::mem::transmute(rhs) };
        *self |= arr;
    }
}

impl<const BYTES: usize> BitAnd<u128> for &Bitmap<BYTES> {
    type Output = u128;
    fn bitand(self, rhs: u128) -> Self::Output {
        let arr: [u8; 16] = unsafe { core::mem::transmute(rhs) };
        let arr = self & arr;
        unsafe { core::mem::transmute(arr) }
    }
}

impl<const BYTES: usize> BitAndAssign<u128> for Bitmap<BYTES> {
    fn bitand_assign(&mut self, rhs: u128) {
        let arr: [u8; 16] = unsafe { core::mem::transmute(rhs) };
        *self &= arr;
    }
}

impl<const BYTES: usize> BitOrAssign<u128> for Bitmap<BYTES> {
    fn bitor_assign(&mut self, rhs: u128) {
        let arr: [u8; 16] = unsafe { core::mem::transmute(rhs) };
        *self |= arr;
    }
}

impl<const BYTES: usize> BitAnd<usize> for &Bitmap<BYTES> {
    type Output = usize;
    fn bitand(self, rhs: usize) -> Self::Output {
        const USIZE: usize = core::mem::size_of::<usize>();
        let arr: [u8; USIZE] = unsafe { core::mem::transmute(rhs) };
        let arr = self & arr;
        unsafe { core::mem::transmute(arr) }
    }
}

impl<const BYTES: usize> BitAndAssign<usize> for Bitmap<BYTES> {
    fn bitand_assign(&mut self, rhs: usize) {
        const USIZE: usize = core::mem::size_of::<usize>();
        let arr: [u8; USIZE] = unsafe { core::mem::transmute(rhs) };
        *self &= arr;
    }
}

impl<const BYTES: usize> BitOrAssign<usize> for Bitmap<BYTES> {
    fn bitor_assign(&mut self, rhs: usize) {
        const USIZE: usize = core::mem::size_of::<usize>();
        let arr: [u8; USIZE] = unsafe { core::mem::transmute(rhs) };
        *self |= arr;
    }
}

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