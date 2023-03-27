pub mod fmt;
pub mod from;
pub mod macros;
pub mod ops;
pub mod refs;
pub mod ptr;
mod traits;

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
/// ## Create a new bitmap using methods
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
/// ## Create a new bitmap using macros (recommend)
/// ```
/// use cbitmap::bitmap::*;
///
/// // Bitmap<0>
/// let map = newmap!();
///
/// // newmap!(;bits) = Bitmap<{(bits + 7) / 8}>
/// let map = newmap!(;35);
///
/// // newmap!(mask:literal | mask:literal | ...; bits)
/// let map = newmap!(1u8 | 0b100000u128; 48);
///
/// // newmap!(var | var | ...; bits)
/// let a = 1u64 << 34;
/// let b = 1u128 << 47;
/// let map = newmap!(a | b; 48);
///
/// // he_lang!(idx | idx | ...; bits)
/// let map = he_lang!(1 | 2; 8);
/// ```
///
/// ## Use immutable methods to inspect bits and bitmap infos
/// ```
/// use cbitmap::bitmap::*;
///
/// let map: Bitmap<1> = newmap!(0b_10000001; 8);
/// // bit_len() can get the length of the bitmap in bits,
/// // while byte_len() get it in bytes:
/// assert_eq!(map.bit_len(), 8);
/// assert_eq!(map.byte_len(), 1);
/// // get_bool() and get_01() can get the value of one bit:
/// assert_eq!(map.get_bool(0), true);
/// assert_eq!(map.get_01(7), 1);
/// // test() is a wrapper of get_bool():
/// assert_eq!(map.test(2), false);
/// // range_to_string() can format a range of bits:
/// let map: Bitmap<2> = newmap!(0b_01_10000000; 16);
/// assert_eq!(&map.range_to_string(4, 10).unwrap(), "01 1000");
/// ```
/// ## Use mutable methods to manipulate bits
/// ```
/// use cbitmap::bitmap::*;
///
/// let mut map = newmap!(;8);
/// // set(), reset() and flip() can modify one bit:
/// assert_eq!(map.test(1), false);
/// map.set(1);
/// assert_eq!(map.test(1), true);
/// map.reset(1);
/// assert_eq!(map.test(1), false);
/// map.flip(1);
/// assert_eq!(map.test(1), true);
/// // set_all(), reset_all() and flip_all() can modify the
/// // whole map:
/// map.set_all();
/// assert_eq!(&map.range_to_string(0, 8).unwrap(), "11111111");
/// // It is also possible to call these methods in chain:
/// map.reset_all().flip(1);
/// assert_eq!(map.test(1), true);
/// ```
/// ## Use wrappers to reference single bits
/// ```
/// use cbitmap::bitmap::*;
///
/// let mut map = newmap!(;8);
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
/// let mut map = newmap!(;8);
///
/// map |= 1u8 << 4;
/// assert_eq!(map.test(4), true);
///
/// map |= 0b_11000000u8;
///
/// // To use &, map shoule be in ref and be on the left:
/// assert_eq!(&map & 0b_11010000_u8, 0b11010000);
///
/// map &= 1u8 << 7;
/// assert_eq!(&map & !0u8, 1u8 << 7);
/// ```
#[derive(Clone)]
pub struct Bitmap<const BYTES: usize> {
    bits: [u8; BYTES],
}

/// A general trait, structs which implemented this 
/// trait provide interfaces to access a range of bits.
pub trait BitsManage {
    fn count(&self) -> usize;

    /// Find the first '1', returns its index.
    /// # Returns
    /// [`None`] if there is no '1', [`Some(usize)`] 
    /// otherwise.
    fn find_first_one(&self) -> Option<usize>;

    /// Find the first '0', returns its index.
    /// # Returns
    /// [`None`] if there is no '0', [`Some(usize)`] 
    /// otherwise.
    fn find_first_zero(&self) -> Option<usize>;
    
    /// Get the bool value of indexed bit.
    /// 
    /// # Panics
    /// Panic if `index` is out of range.
    fn get_bool(&self, index: usize) -> bool;

    /// Set the indexed bit to '1'.
    /// 
    /// # Panics
    /// Panic if `index` is out of range.
    fn set(&mut self, index: usize) -> &mut Self;

    /// Set the indexed bit to '0'.
    /// 
    /// # Panics
    /// Panic if `index` is out of range.
    fn reset(&mut self, index: usize) -> &mut Self;

    /// Flip the indexed bit.
    /// 
    /// # Panics
    /// Panic if `index` is out of range.
    fn flip(&mut self, index: usize) -> &mut Self;

    /// Set all bits to '1'.
    fn set_all(&mut self) -> &mut Self;

    /// Set all bits to '0'.
    fn reset_all(&mut self) -> &mut Self;

    /// Flip all bits.
    fn flip_all(&mut self) -> &mut Self;

    // Default implementations

    /// A wrapper of [`Bitmap::find_first_zero()`].
    ///
    /// # Return
    /// [`bool`]. `true` iff there is no '0' in the
    /// bitmap.
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(0b10;16);
    /// assert_eq!(map.all(), false);
    /// map.set_all();
    /// assert_eq!(map.all(), true);
    /// ```
    #[inline]
    fn all(&self) -> bool {
      self.find_first_zero().is_none()
    }

    /// A wrapper of [`Bitmap::find_first_one()`].
    ///
    /// # Return
    /// [`bool`]. `true` iff there is '1' in the bitmap.
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(;16);
    /// assert_eq!(map.any(), false);
    /// map.set(10);
    /// assert_eq!(map.any(), true);
    /// ```
    #[inline]
    fn any(&self) -> bool {
      self.find_first_one().is_some()
    }

    /// A wrapper of [`Bitmap::find_first_one()`].
    ///
    /// # Return
    /// [`bool`]. `true` iff there is no '1' in
    /// the bitmap.
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(;16);
    /// assert_eq!(map.none(), true);
    /// map.set(10);
    /// assert_eq!(map.none(), false);
    /// ```
    #[inline]
    fn none(&self) -> bool {
      self.find_first_one().is_none()
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
    /// # Panics
    /// Panic if the `index` is out of range.
    #[inline]
    fn get_01(&self, index: usize) -> u8 {
        self.get_bool(index).into()
    }

    /// Get the value of a bit by sepcifying the index.
    /// A wrapper of `get_bool()`.
    ///
    /// # Arguments
    /// * `index`: the index of the bit.
    ///
    /// # Return
    /// [`bool`], the value of the bit.
    ///
    /// # Panics
    /// Panic if the `index` is out of range.
    #[inline]
    fn test(&self, index: usize) -> bool {
        self.get_bool(index)
    }
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
        Bitmap { bits: [0; BYTES] }
    }

    /// Get the length of the bitmap in bits.
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let map = newmap!(;24);
    /// assert_eq!(map.bit_len(), 24);
    /// ```
    #[inline]
    pub const fn bit_len(&self) -> usize {
        BYTES * 8
    }

    /// Get the length of the bitmap in bytes.
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let map = newmap!(;3 * 8);
    /// assert_eq!(map.byte_len(), 3);
    /// ```
    #[inline]
    pub const fn byte_len(&self) -> usize {
        BYTES
    }
}

impl<const BYTES: usize> BitsManage for Bitmap<BYTES> {
    /// Get the value of a bit by sepcifying the index.
    ///
    /// # Arguments
    /// * `index`: the index of the bit.
    ///
    /// # Examples
    /// A simple example:
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let map = newmap!(0b_1; 8);
    /// assert_eq!(map.get_bool(0), true);
    /// ```
    ///
    /// # Panics
    /// Panic if the `index` is out of range.
    #[inline]
    fn get_bool(&self, index: usize) -> bool {
        self.__get_bool(__idx_get_byte(index), __idx_get_bit(index))
    }

    /// Get the minimal index of a '1' in the bitmap.
    ///
    /// # Return
    /// [`Option<usize>`]. [`None`] if there is no '1',
    /// otherwise [`Some(index)`].
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(;16);
    /// assert_eq!(map.find_first_one(), None);
    /// map.set(10);
    /// assert_eq!(map.find_first_one(), Some(10));
    /// map.set(7);
    /// assert_eq!(map.find_first_one(), Some(7));
    /// map.set(0);
    /// assert_eq!(map.find_first_one(), Some(0));
    /// ```
    #[inline]
    fn find_first_one(&self) -> Option<usize> {
        let mut index: usize = 0;
        for b in &self.bits {
            let test = (*b) & (!(*b)).wrapping_add(1);
            index += match test {
                // When test == 0, we jump to next byte.
                0 => 8,
                // Make a list for speed up.
                1 => 0,
                2 => 1,
                4 => 2,
                8 => 3,
                16 => 4,
                32 => 5,
                64 => 6,
                128 => 7,
                // Unreachable.
                _ => panic!("Unexpected arithmetic error"),
            };
            if test > 0 {
                break;
            }
        }
        match index >= BYTES * 8 {
            true => None,
            false => Some(index),
        }
    }

    /// Get the minimal index of a '0' in the bitmap.
    ///
    /// # Return
    /// [`Option<usize>`]. [`None`] if there is no '0',
    /// otherwise [`Some(index)`].
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(;16);
    /// map.set_all();
    /// assert_eq!(map.find_first_zero(), None);
    /// map.reset(10);
    /// assert_eq!(map.find_first_zero(), Some(10));
    /// map.reset(7);
    /// assert_eq!(map.find_first_zero(), Some(7));
    /// map.reset(0);
    /// assert_eq!(map.find_first_zero(), Some(0));
    /// ```
    #[inline]
    fn find_first_zero(&self) -> Option<usize> {
        let mut index: usize = 0;
        for b in &self.bits {
            let test = (*b).wrapping_add(1) & !(*b);
            index += match test {
                // When test == 0, we jump to next byte.
                0 => 8,
                // Make a list for speed up.
                1 => 0,
                2 => 1,
                4 => 2,
                8 => 3,
                16 => 4,
                32 => 5,
                64 => 6,
                128 => 7,
                // Unreachable.
                _ => panic!("Unexpected arithmetic error"),
            };
            if test > 0 {
                break;
            }
        }
        match index >= BYTES * 8 {
            true => None,
            false => Some(index),
        }
    }

    /// Count how many '1's are in the bitmap.
    ///
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(;64);
    /// assert_eq!(map.count(), 0);
    /// let some = [
    ///     0b11, 0b110, 0b101, 0b1010,
    ///     0b1111, 0b1, 0b10000, 0b01];
    /// map.fill_prefix(some);
    /// assert_eq!(map.count(), 15);
    /// map.flip_all();
    /// assert_eq!(map.count(), 49);
    /// map.set_all();
    /// assert_eq!(map.count(), 64);
    /// ```
    #[inline]
    fn count(&self) -> usize {
        let mut cnt: usize = 0;
        for b in &self.bits {
            let mut temp = *b;
            temp = ((temp & 0b10101010) >> 1) + (temp & 0b01010101);
            temp = ((temp & 0b11001100) >> 2) + (temp & 0b00110011);
            temp = (temp >> 4) + (temp & 0b1111);
            cnt += temp as usize;
        }
        cnt
    }

    /// Set a bit to 1 by specifying the index.
    ///
    /// # Return
    /// `&mut self`, allowing a call chain.
    ///
    /// # Examples
    /// Examples including call chain:
    /// ```
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(;8);
    /// map.set(0);
    /// assert_eq!(map.test(0), true);
    ///
    /// map.set(1).set(2).set(3);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "00001111");
    /// ```
    fn set(&mut self, index: usize) -> &mut Self {
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
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(0b_11111111; 8);
    /// map.reset(0);
    /// assert_eq!(map.test(0), false);
    ///
    /// map.reset(1).reset(2).reset(3);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "11110000");
    /// ```
    fn reset(&mut self, index: usize) -> &mut Self {
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
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(;8);
    /// map.flip(0);
    /// assert_eq!(map.test(0), true);
    ///
    /// map.flip(1).flip(0);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "00000010");
    /// ```
    fn flip(&mut self, index: usize) -> &mut Self {
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
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(;8);
    /// map.set_all().flip(1);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "11111101");
    /// ```
    fn set_all(&mut self) -> &mut Self {
        *&mut self.bits = [255; BYTES];
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
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(0b11111111; 8);
    /// map.reset_all().flip(1);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "00000010");
    /// ```
    fn reset_all(&mut self) -> &mut Self {
        *&mut self.bits = [0; BYTES];
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
    /// use cbitmap::bitmap::*;
    ///
    /// let mut map = newmap!(0b_10101010; 8);
    /// map.flip_all();
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "01010101");
    ///
    /// map.flip_all().set(0);
    /// assert_eq!(&map.range_to_string(0, 8).unwrap(), "10101011");
    /// ```
    fn flip_all(&mut self) -> &mut Self {
        let arr = &mut self.bits;
        for i in arr {
            *i = !*i;
        }
        self
    }
}

// Tools

impl<const BYTES: usize> Bitmap<BYTES> {
    #[inline]
    fn __get_bool(&self, byte: usize, bit: usize) -> bool {
        &self.bits[byte] & (1 << bit) != 0
    }

    #[inline]
    fn __copy_u8<'map>(&'map self, byte: usize) -> u8 {
        self.bits[byte].clone()
    }

    #[inline]
    fn __get_mut_u8<'map>(&'map mut self, byte: usize) -> &'map mut u8 {
        &mut self.bits[byte]
    }
}

use crate::tools::inner_use::*;

pub use crate::{he_lang, newmap};
pub use refs::*;
pub use traits::FillPrefix;
