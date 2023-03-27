//! Bit ref wrappers.

use super::*;

/// A wrapper of the immutable reference to a bit in the bitmap.
/// The wrapper owns a `ref` to the map.
///
/// # Examples
/// To get a new wrapper:
/// ```
/// use cbitmap::bitmap::*;
///
/// let mut map = newmap!(;8);
/// let rmap = &map;
///
/// let bit1 = map.at(1);
/// let bit2 = BitRef::<1>::new(rmap, 2);
/// ```
/// Use deref to get the bool value:
/// ```
/// use cbitmap::bitmap::*;
///
/// let rmap = &newmap!(0b_0010; 8);
/// let bit = rmap.at(1);
/// assert_eq!(*bit, true);
/// ```
/// [`Copy`] is implemented:
/// ```
/// use cbitmap::bitmap::*;
///
/// let rmap = &newmap!(1; 8);
/// let bit1 = rmap.at(0);
/// let bit2 = bit1;
/// // bit1 is copied but not moved to bit2
/// assert_eq!(*bit1, true);
/// ```
/// Convert to [`bool`] use [`Into<bool>`]:
/// ```
/// use cbitmap::bitmap::*;
/// let map = newmap!(;8);
/// let bit = map.at(1);
/// assert_eq!(Into::<bool>::into(bit), false);
/// // Panic! Since bitmut is already moved:
/// // assert_eq!(Into::<bool>::into(bit), false);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct BitRef<'map, const BYTES: usize> {
    pub(super) value: bool,
    _map: &'map Bitmap<BYTES>,
}

/// A wrapper of the mutable reference to a bit in the bitmap.
/// The wrapper owns a `mut ref` to the map.
///
/// # Examples
/// To get a new wrapper:
/// ```
/// use cbitmap::bitmap::*;
///
/// let mut map = newmap!(;8);
/// let mut rmap = &mut map;
///
/// {
///     let mut bitmut1 = rmap.at_mut(2);
/// }
/// {
///     let mut bitmut2 = BitRefMut::<1>::new(rmap, 2);
/// }
/// ```
/// Use deref to get the bool value:
/// ```
/// use cbitmap::bitmap::*;
///
/// let mut map = newmap!(0b_0100; 8);
/// let mut bitmut = map.at_mut(2);
/// assert_eq!(*bitmut, true);
/// ```
/// Use the methods `set()`, `reset()` and `flip()` to change the bit:
/// ```
/// use cbitmap::bitmap::*;
///
/// let mut map = newmap!(;8);
/// let mut bitmut = map.at_mut(1);
/// bitmut.set();
/// assert_eq!(*bitmut, true);
/// // We also allow call chain:
/// bitmut.reset().flip();
/// assert_eq!(*bitmut, true);
/// ```
/// Convert to [`bool`] use [`Into<bool>`]:
/// ```
/// use cbitmap::bitmap::*;
/// let mut map = newmap!(;8);
/// let mut bitmut = map.at_mut(1);
/// assert_eq!(Into::<bool>::into(bitmut), false);
/// // Panic! Since bitmut is already moved:
/// // assert_eq!(Into::<bool>::into(bitmut), false);
/// ```
#[derive(Debug)]
pub struct BitRefMut<'map, const BYTES: usize> {
    idx: usize,
    pub(super) value: bool,
    map: &'map mut Bitmap<BYTES>,
}

impl<'map, const BYTES: usize> BitRef<'map, BYTES> {
    /// Manually create a `BitRef` by specifying the map and index.
    ///
    /// # Panics
    /// Panic if `index` is out of range.
    pub fn new(map: &'map Bitmap<BYTES>, index: usize) -> Self {
        if __out_bound(BYTES, index) {
            panic!("Bitmap: indexing out of range");
        }
        let (byte, bit) = __idx_1dto2d(index);
        Self {
            value: map.__get_bool(byte, bit),
            _map: map,
        }
    }
}

impl<'map, const BYTES: usize> BitRefMut<'map, BYTES> {
    #[inline]
    fn __get_byte(&mut self) -> &mut u8 {
        let by = __idx_get_byte(self.idx);
        self.map.__get_mut_u8(by)
    }

    /// Manually create a `BitRefMut` by specifying the map and index.
    ///
    /// # Panics
    /// Panic if `index` is out of range.
    pub fn new(map: &'map mut Bitmap<BYTES>, index: usize) -> Self {
        if __out_bound(BYTES, index) {
            panic!("Bitmap: indexing out of range");
        }
        let (byte, bit) = __idx_1dto2d(index);
        let value = map.__get_bool(byte, bit);
        Self {
            idx: index,
            value: value,
            map: map,
        }
    }

    /// Set the bit referenced by `self` to 1.
    ///
    /// # Return
    /// `&mut self`, allowing a call chain.
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(;8);
    /// {
    ///     let mut bitmut = map.at_mut(4);
    ///     bitmut.set();
    /// }
    /// assert_eq!(map.test(4), true);
    /// ```
    pub fn set(&mut self) -> &mut Self {
        let mask = 1 << (__idx_get_bit(self.idx));
        __byte_or_u8(self.__get_byte(), mask);
        self.value = true;
        self
    }

    /// Set the bit referenced by `self` to 0.
    ///
    /// # Return
    /// `&mut self`, allowing a call chain.
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(0b11111111; 8);
    /// {
    ///     let mut bitmut = map.at_mut(4);
    ///     bitmut.reset();
    /// }
    /// assert_eq!(map.test(4), false);
    /// ```
    pub fn reset(&mut self) -> &mut Self {
        let mask = !(1 << (__idx_get_bit(self.idx)));
        __byte_and_u8(self.__get_byte(), mask);
        self.value = false;
        self
    }

    /// Flip the bit referenced by `self`.
    ///
    /// # Return
    /// `&mut self`, allowing a call chain.
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(;8);
    /// {
    ///     let mut bitmut = map.at_mut(4);
    ///     bitmut.flip();
    ///     assert_eq!(*bitmut, true);
    ///     bitmut.flip();
    ///     assert_eq!(*bitmut, false);
    /// }
    /// assert_eq!(map.test(4), false);
    /// ```
    pub fn flip(&mut self) -> &mut Self {
        let mask = 1 << __idx_get_bit(self.idx);
        let byte = self.__get_byte();
        *byte ^= mask;
        self.value = !self.value;
        self
    }
}

impl<const BYTES: usize> Bitmap<BYTES> {
    /// Get the immutable reference of the indexed bit in the bitmap.
    /// The bit is wrapped in `BitRef`.
    ///
    /// One can deref this reference to get the bool value of the bit.
    ///
    /// # Examples
    /// Simple examples:
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let map = newmap!(0b_00001000; 8);
    /// let bit = map.at(3);
    /// assert_eq!(*bit, true);
    /// ```
    ///
    /// # Panics
    /// Panic if `index` is out of range.
    pub fn at<'map>(&'map self, index: usize) -> BitRef<'map, BYTES> {
        BitRef::new(self, index)
    }

    /// Get the mutable reference of the indexed bit in the bitmap.
    /// The bit is wrapped in `BitRefMut`.
    ///
    /// One can deref this reference to get the bool value of the bit,
    /// or use the methods to modify the referenced bit.
    ///
    /// # Examples
    /// Simple examples:
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(0b_00001000; 8);
    /// {
    ///     let mut bitmut = map.at_mut(3);
    ///     assert_eq!(*bitmut, true);
    ///     bitmut.reset();
    ///     assert_eq!(*bitmut, false);
    /// }
    /// ```
    ///
    /// # Panics
    /// Panic if `index` is out of range.
    pub fn at_mut<'map>(&'map mut self, index: usize) -> BitRefMut<'map, BYTES> {
        BitRefMut::new(self, index)
    }
}
