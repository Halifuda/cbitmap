//! Providing macros to create bitmap convenently.

/// Create a `cbitmap::bitmap::Bitmap` by specifying the bit length and flags.
/// 
/// # Examples
/// Create a no-bit bitmap `cbitmap::bitmap::Bitmap<0>`:
/// ```
/// use cbitmap::bitmap::*;
/// let map = newmap!();
/// assert_eq!(map.bit_len(), 0);
/// ```
/// Create a default bitmap with all zero bit, specifying its
/// (expected) bit length. The actual length will be rounded up:
/// ```
/// use cbitmap::bitmap::*;
/// // must add ';' to indicate the argument is specifying length
/// let map = newmap!(;34);
/// // length will be rounded up to a multiple of 8:
/// assert_eq!(map.bit_len(), 40);
/// // you can also use const exprs:
/// let map = newmap!(;4096 * 8);
/// assert_eq!(map.byte_len(), 4096);
/// ```
/// Create a bitmap with flags. The flags muts be literal 
/// integers, and are enumerated with `|`. The length is still 
/// required to be specified:
/// ```
/// use cbitmap::bitmap::*;
/// let map = newmap!(1u8 | 0b100000u128; 8);
/// assert_eq!(map.test(0), true);
/// assert_eq!(map.test(5), true);
/// ```
/// You can also use variables, but you cannot use exprs:
/// ```
/// use cbitmap::bitmap::*;
/// let a = 1u64 << 34;
/// let b = 1u128 << 47;
/// let map = newmap!(a | b; 48);
/// assert_eq!(map.test(34), true);
/// assert_eq!(map.test(47), true);
/// // Not allowed!
/// // let map = newmap!((1 << 12) | (1 << 13); 14);
/// ```
/// # See also
/// [`crate::he_lang`]
#[macro_export]
macro_rules! newmap {
    () => {
        Bitmap::<0>::new()
    };
    (;$n:expr) => {
        Bitmap::<{($n + 7) >> 3}>::new()
    };
    (
        $a:literal
        ;$n:expr
    ) => {
        {
            let mut map = Bitmap::<{(($n) + 7) >> 3}>::new();
            map |= $a;
            map
        }
    };
    (
        $($a:literal)|*$(|)?
        ;$n:expr
    ) => {
        {
            let mut map = Bitmap::<{(($n) + 7) >> 3}>::new();
            $(
                map |= $a;
            )*
            map
        }
    };
    (
        $($a:ident)|*$(|)?
        ;$n:expr
    ) => {
        {
            let mut map = Bitmap::<{(($n) + 7) >> 3}>::new();
            $(
                map |= $a;
            )*
            map
        }
    };
}

/// A wrapper of [`newmap`], which is a painted eggshell. 
/// Create a bitmap with indexes instead of flags. 
/// 
/// # Examples
/// It is allowed to use literal integers and variables:
/// ```
/// use cbitmap::bitmap::*;
/// let map = he_lang!(1 | 4; 5);
/// assert_eq!(map.bit_len(), 8);
/// assert_eq!(map.test(1), true);
/// assert_eq!(map.test(4), true);
/// ```
/// 
#[macro_export]
macro_rules! he_lang {
    (
        $($a:literal)|*$(|)?
        ;$n:expr
    ) => {
        {
            let mut map = Bitmap::<{(($n) + 7) >> 3}>::new();
            map$(.set($a))*;
            map
        }
    };
}