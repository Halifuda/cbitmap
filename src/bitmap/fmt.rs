//! Implementations of formating methods for `Bitmap`, including [`Debug`].

#[cfg(feature = "alloc")]
use alloc::{string::{String, ToString}, format};

use crate::bitmap::*;

#[cfg(not(feature = "alloc"))]
impl<const BYTES: usize> core::fmt::Display for Bitmap<BYTES> {
    /// Formats a bitmap. Only shows the last 2 bytes if the bitmap is longer.
    /// The bytes will be separated by space `' '`. 
    /// 
    /// The bits will be arranged from right to left. If the bitmap is longer than 
    /// 2 bytes, a `"..."` will show on the left.
    /// On the very left, a bracket tells the bit length of the map (in a form 
    /// like `"[N bits]"`). A space `' '` will be between the bit contents and this
    /// bracket.
    /// 
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    /// 
    /// let mut map: Bitmap<3> = 0.into();
    /// map.set(0);
    /// map.set(8);
    /// let str = &format!("{map}");
    /// assert_eq!(str, "[24 bits] ...00000001 00000001");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{} bits] ", BYTES * 8)?;
        let size = 2.min(BYTES);
        if BYTES > size {
            write!(f, "...")?;
        }
        for i in 0..size {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{:08b}", self.__copy_u8(size - i - 1))?;
        }
        Ok(())
    }
}

#[cfg(not(feature = "alloc"))]
impl<const BYTES: usize> core::fmt::Debug for Bitmap<BYTES> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Bitmap")
            .field("#bytes", &BYTES)
            .field("#bits", &(BYTES * 8))
            .field("bits", &self.bits)
            .finish()
    }
}

#[cfg(feature = "alloc")]
impl<const BYTES: usize> core::fmt::Display for Bitmap<BYTES> {
    /// Formats a bitmap. Only shows the last 2 bytes if the bitmap is longer.
    /// The bytes will be separated by space `' '`. 
    /// 
    /// The bits will be arranged from right to left. If the bitmap is longer than 
    /// 2 bytes, a `"..."` will show on the left.
    /// On the very left, a bracket tells the bit length of the map (in a form 
    /// like `"[N bits]"`). A space `' '` will be between the bit contents and this
    /// bracket.
    /// 
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    /// 
    /// let mut map: Bitmap<3> = 0.into();
    /// map.set(0);
    /// map.set(8);
    /// let str = &format!("{map}");
    /// assert_eq!(str, "[24 bits] ...00000001 00000001");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut contents = String::new();
        contents.push_str(&format!("[{} bits] ", BYTES * 8));
        let size = 2.min(BYTES);
        if BYTES > size {
            contents.push_str("...")
        }
        for i in 0..size {
            if i > 0 {
                contents.push_str(" ");
            }
            contents.push_str(&format!("{:08b}", self.__copy_u8(size - i - 1)));
        }
        write!(f, "{contents}")
    }
}

#[cfg(feature = "alloc")]
impl<const BYTES: usize> core::fmt::Debug for Bitmap<BYTES> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut contents = String::new();
        let size = 2.min(BYTES);
        if BYTES > size {
            contents.push_str("...")
        }
        for i in 0..size {
            if i > 0 {
                contents.push_str(" ");
            }
            contents.push_str(&format!("{:08b}", self.__copy_u8(size - i - 1)));
        }

        f.debug_struct("Bitmap")
            .field("#bytes", &BYTES)
            .field("#bits", &(BYTES * 8))
            .field("bits", &contents)
            .finish()
    }
}

#[cfg(feature = "alloc")]
impl<const BYTES: usize> Bitmap<BYTES> {
    /// Format a range of bits into a [`Option<String>`].
    /// 
    /// An array of `'0'`/`'1'` will show in the String. The bits are separated 
    /// by `' '` at the edge of 2 bytes. 
    /// 
    /// # Return
    /// [`None`] if the range is invalid (out of the bitmap, or length is less 
    /// than 0), [`Some(String)`] otherwise.
    /// 
    /// # Examples
    /// ```
    /// use cbitmap::bitmap::*;
    /// 
    /// let map: Bitmap<2> = 0b_011_11001100.into();
    /// let string = map.range_to_string(2, 11).unwrap();
    /// assert_eq!(&string, "011 110011");
    /// 
    /// assert!(map.range_to_string(0, 100).is_none());
    /// assert!(map.range_to_string(100, 101).is_none());
    /// assert!(map.range_to_string(2, 1).is_none());
    /// ```
    pub fn range_to_string(&self, start: usize, end: usize) -> Option<String> {
        if start >= end || __out_bound(BYTES, start) || __out_bound(BYTES, end - 1) {
            return None;
        }

        let mut contents = String::new();
        let mut i = end - 1;
        while __idx_get_byte(i) > __idx_get_byte(start) {
            let bit = __idx_get_bit(i);
            match bit {
                7 => {
                    contents.push_str(&format!("{:08b}", self.__copy_u8(__idx_get_byte(i))));
                    contents.push_str(" ");
                    i -= 8;
                }
                _ => {
                    contents.push_str(&self.get_01(i).to_string());
                    if bit == 0 {
                        contents.push_str(" ");
                    }
                    i -= 1;
                }
            }
        }
        while i > start {
            contents.push_str(&self.get_01(i).to_string());
            i -= 1;
        }
        contents.push_str(&self.get_01(i).to_string());

        Some(contents)
    }
}
