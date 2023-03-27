//! # cbitmap
//! 
//!  > A crate of conventional, compact and core (no_std) bitmap.
//! 
//! ## Use cases
//! 
//! You are recommended to use this crate when you want to 
//! maintain a bitmap containing a large but fixed number of bits. 
//! Especially, when you are caring memory usage/alignment, for 
//! `cbitmap` wastes almost no places.
//! 
//! For example, you may want to manage a set of resources, which 
//! can be described by two states, and a bitmap is fit for you. 
//! 
//! If you want to maintain a small set of flags, like 2 or 3, we
//! recommend [flagset](https://crates.io/crates/flagset) instead.
//! 
//! The most extensive and mature implementation of bitmap might be 
//! [bitvec](https://crates.io/crates/bitvec). You are recommended
//！ to use it if you are caring about maturity. 
//!
//! Also, [bitset-core](https://crates.io/crates/bitset-core) is 
//! an earlier yet powerful crate that implemented bitset trait. 
//! However, the implementation between `bitset-core` and `cbitmap` 
//! is quiet different. 
//! 
//! What's more, the performance of `cbitmap` has not been tested 
//! since it's on alpha version. If you care most about performance, 
//! please make a careful consideration before choice.
//! 
//! ## Features
//! 
//! We provided a [`crate::bitmap::Bitmap`] type:
//! 
//! ```ignore
//! pub struct Bitmap<const BYTES: usize> {
//!     bits: [u8; BYTES],
//! }
//! ```
//! 
//! You are recommended to use macros to create new bitmaps:
//! 
//! ```
//! use cbitmap::bitmap::*;
//! 
//! let map = newmap!(0b_01; 2);
//! ```
//! 
//! See also [`crate::he_lang`].
//! 
//! The bitmap can be manipulated in conventional ways, like 
//! [`crate::bitmap::BitsManage::test()`], 
//! [`crate::bitmap::BitsManage::set()`], 
//! [`crate::bitmap::BitsManage::reset()`], 
//! [`crate::bitmap::BitsManage::flip()`] and 
//! [`crate::bitmap::Bitmap::at()`]. 
//! 
//! The bitmap is actually a wrapper of [`u8`] array `[u8; BYTES]`.
//! It can be put on diverse places on memory. 
//! For example, if the map is relatively small like 8 or 16 bits, 
//! you can put it on stack safely. If it is larger like 256 or 
//! 1024 bits, you may want to put it on heap. 
//! 
//! ## Examples
//! 
//! Here is a simple example:
//! 
//! ```
//! use cbitmap::bitmap::*;
//! 
//! // A macro are provided to create a bitmap.
//! let mut map = newmap!(;16);
//! 
//! // There is a set of methods to manipulate the bitmap:
//! map.set(10);
//! map.reset(10);
//! map.flip(10);
//! 
//! // Some C++ like methods are provided:
//! assert_eq!(map.test(10), true);
//! assert_eq!(map.any(), true);
//! assert_eq!(map.none(), false);
//! 
//! // Also provide other useful methods:
//! assert_eq!(map.find_first_one(), Some(10));
//! 
//! // You can access a single bit using wrappers:
//! let mut bit = map.at_mut(10);
//! assert_eq!(*bit, true);
//! bit.flip();
//! assert_eq!(*map.at(10), false);
//! ```
//! 
//! Please see the documentation of [`crate::bitmap::Bitmap`] and 
//! the examples dir for detailed examples.
//! 
//! You can use `cargo run --example <name>` to run the examples we 
//! provide. A simple example is `bitmap-base`, another extensive 
//! example about practical usage is `bitmap-usecase`, where bitmap 
//! is used to manage raw memory resources.
//! 
//! ## Current constraints
//! 
//! ### Generic const expr
//! 
//! The bitmap is specified with its size-in-bytes by `BYTES`. This 
//! is slightly different from conventional `bitset<N>` in C++, 
//! where `N` indicating the size-in-bits. We implemented bitmap 
//! in this way to stay on rust-stable, where the 
//! `#![feature(generic_const_exprs)]` is not supported yet, thus, 
//! it is not allowed to do like this:
//! 
//! ```ignore
//! // requiring #![feature(generic_const_exprs)]
//! pub struct Bitmap<const N: usize> {
//!     bits: [u8; (N + 7) / 8],
//! }
//! ```
//! 
//! We have provided an alternative way to let you specify the size
//! in bits. The macro `crate::newmap` achieves this:

//! ```ignore
//! const BITS: usize = 16;
//! let map = newmap!(;BITS);
//! let another = newmap!(;BITS * 2);
//! ```
//!
//! In principle, it is nevertheless possible to use constexpr when 
//! instantiating a struct:
//!
//! ```ignore
//! // allowed:
//! let map = Bitmap::<{64 / 8}>::new();
//! ```
//! 
//! ### Index
//! 
//! A `bitset<N>` in C++ can be indexed by Index op `[]`. We have 
//! met some problems when implementing this feature. Specifically, 
//! implementing [`core::ops::IndexMut`] for a struct is like this:
//! 
//! ```ignore
//! impl IndexMut for T {
//!     type Output = U;
//!     fn index(&mut self, index: usize) -> &mut Self::Output { ... }
//! }
//! ```
//! 
//! The ref in `&mut Self::Output` requires `self` to own the indexed output. 
//! 
//! In [`crate::bitmap::Bitmap`], `Output` is required to be "bits". 
//! It is necessary to use a wrapper type to provide interfaces to 
//! access a single bits. We have provided [`crate::bitmap::BitRef`] and 
//! [`crate::bitmap::BitRefMut`] as the wrappers. 
//! 
//! However, the bitmap is not expected to hold a large set of wrappers,
//! in order to save memories.
//!  
//! Due to this issue, we only provide  [`crate::bitmap::Bitmap::at_mut()`] 
//! as methods to multably index into the bitmap.
//!
//! It is noteworthy that, we provide [`crate::bitmap::Bitmap::at()`] to get 
//! [`crate::bitmap::BitRef`], and we also provide immutable [`core::ops::Index`]. 
//! However, immutable [`core::ops::Index`] only returns a `bool` value, 
//! not `BitRef` due to a similar issue.
#![no_std]

extern crate alloc;

mod tools;
pub mod bitmap;