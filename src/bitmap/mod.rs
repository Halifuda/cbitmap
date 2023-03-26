pub mod from;
pub mod ops;
pub mod fmt;

#[derive(Clone)]
/// A size-fixed bitmap with croase-granularity (byte) and conventional 
/// interfaces. 
/// 
/// # Generics
/// * `BYTES`: a [`usize`], specifying the byte length of the bitmap.
/// 
/// **Note: The actual number of flags (bits) will be `BYTES * 8`.**
/// 
/// # Fields
/// * `bits`: [`Option<Box>`]. The box holds an array of [`u8`] with 
/// fixed length `BYTES`. 
/// 
/// It is allowed to have `BYTES == 0`. In this case, `bits = None`.
/// 
/// # Examples
/// ## Create a new bitmap
/// ```
/// use cbitmap::bitmap::*;
/// 
/// // new() will create a bitmap, init the flags to 0:
/// let map = Bitmap::<1>::new();
/// // Or use Default::default():
/// let map: Bitmap<1> = Default::default();
/// 
/// // You can init a map with a [u8; BYTES]:
/// let map: Bitmap<1> = [0b_10_u8; 1].into();
/// let map = Bitmap::<1>::from([0b_100_u8; 1]);
/// 
/// // You can also init a map with an unsized integer. The exact 
/// // type of the integer can be omitted, but the `BYTES` must be 
/// // specified:
/// let map: Bitmap<2> = 0b_10000000_00000001.into();
/// ```
/// ## Use immutable methods to inspect bits
/// ```
/// use cbitmap::bitmap::*;
/// 
/// let map: Bitmap<1> = 0b_10000001.into();
/// // get_bool() and get_01() can get the value of one bit:
/// assert_eq!(map.get_bool(0), true);
/// assert_eq!(map.get_01(7), 1);
/// // range_to_string() can format a range of bits:
/// let map: Bitmap<2> = 0b_01_10000000.into();
/// assert_eq!(&map.range_to_string(4, 10).unwrap(), "01 1000");
/// ```
/// ## Use mutable methods to manipulate bits
/// ```
/// use cbitmap::bitmap::*;
/// 
/// let mut map: Bitmap<1> = 0.into();
/// // set(), reset() and flip() can modify one bit:
/// assert_eq!(map.get_bool(1), false);
/// map.set(1);
/// assert_eq!(map.get_bool(1), true);
/// map.reset(1);
/// assert_eq!(map.get_bool(1), false);
/// map.flip(1);
/// assert_eq!(map.get_bool(1), true);
/// // set_all(), reset_all() and flip_all() can modify the 
/// // whole map:
/// map.set_all();
/// assert_eq!(&map.range_to_string(0, 8).unwrap(), "11111111");
/// // It is also possible to call these methods in chain:
/// map.reset_all().flip(1);
/// assert_eq!(map.get_bool(1), true);
/// ```
/// ## Use wrappers to reference single bits
/// ```
/// use cbitmap::bitmap::*;
/// 
/// let mut map: Bitmap<1> = 0.into();
/// // at() can give a immutable ref of a bit, with a wrapper:
/// let bit0 = map.at(0);
/// let bit1 = map.at(1);
/// 
/// // Panic! Since map has already referenced by bit0 and bit1
/// // map.set(1);
/// 
/// // Use deref to peek the bit:
/// assert_eq!(*bit0, false);
/// 
/// {
///     let mut bitmut = map.at_mut(1);
///     bitmut.set();
/// }
/// assert_eq!(*map.at(1), true);
/// ```
/// ## Use bitwise operators
/// ```
/// use cbitmap::bitmap::*;
/// 
/// let mut map: Bitmap<1> = 0.into();
/// 
/// map |= 1u8 << 4;
/// assert_eq!(map.get_bool(4), true);
/// 
/// map |= 0b_11000000u8;
/// 
/// // To use &, map shoule be in ref and be on the left:
/// assert_eq!(&map & 0b_11010000_u8, 0b11010000);
/// 
/// map &= 1u8 << 7;
/// assert_eq!(&map & !0u8, 1u8 << 7);
/// ```
pub struct Bitmap<const BYTES: usize> {
    bits: Option<[u8; BYTES]>,
}

#[derive(Debug, Clone, Copy)]
/// A wrapper of the immutable reference to a bit in the bitmap.
/// The wrapper owns a `ref` to the map.
/// 
/// # Examples
/// To get a new wrapper:
/// ```
/// use cbitmap::bitmap::*;
/// 
/// let mut map = Bitmap::<1>::new();
/// let rmap = &map;
/// 
/// let bit1 = map.at(1);
/// let bit2 = BitRef::<1>::new(rmap, 2);
/// ```
/// Use deref to get the bool value:
/// ```
/// use cbitmap::bitmap::*;
/// 
/// let rmap: &Bitmap<1> = &0b_0010.into();
/// let bit = rmap.at(1);
/// assert_eq!(*bit, true);
/// ```
/// [`Copy`] is implemented:
/// ```
/// use cbitmap::bitmap::*;
/// 
/// let rmap: &Bitmap<1> = &1.into();
/// let bit1 = rmap.at(0);
/// let bit2 = bit1;
/// // bit1 is copied but not moved to bit2
/// assert_eq!(*bit1, true);
/// ```
/// Convert to [`bool`] use [`Into<bool>`]:
/// ```
/// use cbitmap::bitmap::*;
/// let map: Bitmap<1> = 0.into();
/// let bit = map.at(1);
/// assert_eq!(Into::<bool>::into(bit), false);
/// // Panic! Since bitmut is already moved:
/// // assert_eq!(Into::<bool>::into(bit), false);
/// ```
pub struct BitRef<'map, const BYTES: usize> {
    value: bool,
    _map: &'map Bitmap<BYTES>,
}

#[derive(Debug)]
/// A wrapper of the mutable reference to a bit in the bitmap.
/// The wrapper owns a `mut ref` to the map.
/// 
/// # Examples
/// To get a new wrapper:
/// ```
/// use cbitmap::bitmap::*;
/// 
/// let mut map = Bitmap::<1>::new();
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
/// let mut map: Bitmap<1> = 0b_0100.into();
/// let mut bitmut = map.at_mut(2);
/// assert_eq!(*bitmut, true);
/// ```
/// Use the methods `set()`, `reset()` and `flip()` to change the bit:
/// ```
/// use cbitmap::bitmap::*;
/// 
/// let mut map: Bitmap<1> = 0.into();
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
/// let mut map: Bitmap<1> = 0.into();
/// let mut bitmut = map.at_mut(1);
/// assert_eq!(Into::<bool>::into(bitmut), false);
/// // Panic! Since bitmut is already moved:
/// // assert_eq!(Into::<bool>::into(bitmut), false);
/// ```
pub struct BitRefMut<'map, const BYTES: usize> {
    idx: usize,
    value: bool,
    map: &'map mut Bitmap<BYTES>,
}

impl<const BYTES: usize> Bitmap<BYTES> {
    /// Create a `Bitmap<BYTES>` whose flags are all set to 0.
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    ///
    /// let map = Bitmap::<2>::new();
    /// assert_eq!(&map.range_to_string(0, 16).unwrap(), 
    ///            "00000000 00000000");
    /// ```
    pub fn new() -> Self {
        Bitmap {
            bits: match BYTES == 0 {
                true => None,
                false => Some([0; BYTES]),
            },
        }
    }

    /// Get the value of a bit by sepcifying the index.
    /// 
    /// # Arguments
    /// * `index`: the index of the bit.
    /// 
    /// # Examples
    /// A simple example:
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let map: Bitmap<1> = 1.into();
    /// assert_eq!(map.get_bool(0), true);
    /// ```
    /// 
    /// If the map has 0 byte, `get_bool()` will always return `false`, 
    /// no matter the `index`.
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let map = Bitmap::<0>::new();
    /// assert_eq!(map.get_bool(345), false);
    /// ```
    /// 
    /// # Panics
    /// Panic if the `index` is out of range.
    pub fn get_bool(&self, index: usize) -> bool {
        self.__get_bool(__idx_get_byte(index), __idx_get_bit(index))
    }

    /// Get the value of a bit by sepcifying the index. 
    /// A wrapper of `get_bool()`.
    /// 
    /// # Arguments
    /// * `index`: the index of the bit.
    /// 
    /// # Return
    /// `0/1`, the exact value of the bit.
    /// 
    /// # Examples
    /// A simple example:
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let map: Bitmap<1> = 1.into();
    /// assert_eq!(map.get_01(0), 1);
    /// ```
    /// 
    /// If the map has 0 byte, `get_01()` will always return `0`, 
    /// no matter the `index`.
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let map = Bitmap::<0>::new();
    /// assert_eq!(map.get_01(345), 0);
    /// ```
    /// 
    /// # Panics
    /// Panic if the `index` is out of range.
    pub fn get_01(&self, index: usize) -> u8 {
        match self.get_bool(index) {
            true => 1,
            false => 0,
        }
    }

    /// Get the immutable reference of the indexed bit in the bitmap. 
    /// The bit is wrapped in `BitRef`. 
    /// 
    /// One can deref this reference to get the bool value of the bit.
    /// 
    /// # Examples
    /// Simple examples:
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let map: Bitmap<1> = 0b00001000.into();
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
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let mut map: Bitmap<1> = 0b00001000.into();
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

    /// Set a bit to 1 by specifying the index.
    /// 
    /// # Return
    /// `&mut self`, allowing a call chain.
    /// 
    /// # Examples
    /// Examples including call chain:
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let mut map: Bitmap<1> = 0.into();
    /// map.set(0);
    /// assert_eq!(map.get_bool(0), true);
    /// 
    /// map.set(1).set(2).set(3);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "00001111");
    /// ```
    pub fn set(&mut self, index: usize) -> &mut Self {
        if __out_bound(BYTES, index) {
            panic!("Bitmap: setting out of range");
        }
        let content = 1u8 << __idx_get_bit(index);
        let byte = self.__get_mut_u8(__idx_get_byte(index));
        __byte_or_u8(byte, content);
        self
    }

    /// Set a bit to 0 by specifying the index.
    /// 
    /// # Return
    /// `&mut self`, allowing a call chain.
    /// 
    /// # Examples
    /// Examples including call chain:
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let mut map: Bitmap<1> = 0b11111111.into();
    /// map.reset(0);
    /// assert_eq!(map.get_bool(0), false);
    /// 
    /// map.reset(1).reset(2).reset(3);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "11110000");
    /// ```
    pub fn reset(&mut self, index: usize) -> &mut Self {
        if __out_bound(BYTES, index) {
            panic!("Bitmap: resetting out of range");
        }
        let content = !(1u8 << __idx_get_bit(index));
        let byte = self.__get_mut_u8(__idx_get_byte(index));
        __byte_and_u8(byte, content);
        self
    }

    /// Flip a bit by specifying the index.
    /// 
    /// # Return
    /// `&mut self`, allowing a call chain.
    /// 
    /// # Examples
    /// Examples including call chain:
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let mut map: Bitmap<1> = 0.into();
    /// map.flip(0);
    /// assert_eq!(map.get_bool(0), true);
    /// 
    /// map.flip(1).flip(0);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "00000010");
    /// ```
    pub fn flip(&mut self, index: usize) -> &mut Self {
        if __out_bound(BYTES, index) {
            panic!("Bitmap: flipping out of range");
        }
        let byte = self.__get_mut_u8(__idx_get_byte(index));
        *byte ^= 1 << __idx_get_bit(index);
        self
    }
    
    /// Set the whole map to 1.
    /// 
    /// # Return
    /// `&mut self`, allowing a call chain.
    /// 
    /// # Examples
    /// Examples including call chain:
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let mut map: Bitmap<1> = 0.into();
    /// map.set_all().flip(1);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "11111101");
    /// ```
    pub fn set_all(&mut self) -> &mut Self {
        self.bits.replace([255; BYTES]);
        self
    }

    /// Set the whole map to 0.
    /// 
    /// # Return
    /// `&mut self`, allowing a call chain.
    /// 
    /// # Examples
    /// Examples including call chain:
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let mut map: Bitmap<1> = 0b11111111.into();
    /// map.reset_all().flip(1);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "00000010");
    /// ```
    pub fn reset_all(&mut self) -> &mut Self {
        self.bits.replace([0; BYTES]);
        self
    }

    /// Flip the whole map.
    /// 
    /// # Return
    /// `&mut self`, allowing a call chain.
    /// 
    /// # Examples
    /// Examples including call chain:
    /// ```
    /// use cbitmap::bitmap::Bitmap;
    /// 
    /// let mut map: Bitmap<1> = 0b_10101010.into();
    /// map.flip_all();
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "01010101");
    /// 
    /// map.flip_all().set(0);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "10101011");
    /// ```
    pub fn flip_all(&mut self) -> &mut Self {
        if self.bits.is_some() {
            let arr = self.bits.as_mut().unwrap();
            for i in arr {
                *i = !*i;
            }
        }
        self
    }
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
    /// use cbitmap::bitmap::{Bitmap, BitRefMut};
    ///
    /// let mut map: Bitmap<1> = 0.into();
    /// {
    ///     let mut bitmut = map.at_mut(4);
    ///     bitmut.set();
    /// }
    /// assert_eq!(map.get_bool(4), true);
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
    /// use cbitmap::bitmap::{Bitmap, BitRefMut};
    ///
    /// let mut map: Bitmap<1> = 0b11111111.into();
    /// {
    ///     let mut bitmut = map.at_mut(4);
    ///     bitmut.reset();
    /// }
    /// assert_eq!(map.get_bool(4), false);
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
    /// use cbitmap::bitmap::{Bitmap, BitRefMut};
    ///
    /// let mut map: Bitmap<1> = 0.into();
    /// {
    ///     let mut bitmut = map.at_mut(4);
    ///     bitmut.flip();
    ///     assert_eq!(*bitmut, true);
    ///     bitmut.flip();
    ///     assert_eq!(*bitmut, false);
    /// }
    /// assert_eq!(map.get_bool(4), false);
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
    #[inline]
    fn __get_bool(&self, byte: usize, bit: usize) -> bool {
        match &self.bits {
            None => false,
            Some(arr) => (arr[byte] & (1 << bit)) != 0,
        }
    }

    #[inline]
    fn __copy_u8<'map>(&'map self, byte: usize) -> u8 {
        match &self.bits {
            None => panic!("Bitmap::__copy_u8: meet None"),
            Some(b) => (b[byte]).clone(),
        }
    }

    #[inline]
    fn __get_mut_u8<'map>(&'map mut self, byte: usize) -> &'map mut u8 {
        match &mut self.bits {
            None => panic!("Bitmap::__get_mut_u8: meet None"),
            Some(b) => &mut b[byte],
        }
    }
}

#[inline]
fn __byte_or_u8(byte: &mut u8, mask: u8) {
    *byte |= mask;
}

#[inline]
fn __byte_and_u8(byte: &mut u8, mask: u8) {
    *byte &= mask;
}

#[inline]
fn __idx_get_byte(index: usize) -> usize {
    index >> 3
}

#[inline]
fn __idx_get_bit(index: usize) -> usize {
    index & 0b111
}

#[inline]
fn __idx_1dto2d(index: usize) -> (usize, usize) {
    (__idx_get_byte(index), __idx_get_bit(index))
}

#[inline]
fn __out_bound(bytes: usize, index: usize) -> bool {
    __idx_get_byte(index) >= bytes
}
