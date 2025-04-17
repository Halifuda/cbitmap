[![Build Status](https://github.com/Halifuda/cbitmap/workflows/Rust/badge.svg)](https://github.com/Halifuda/cbitmap/actions)
[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crate](https://img.shields.io/crates/v/cbitmap.svg)](https://crates.io/crates/cbitmap)
[![Doc](https://docs.rs/cbitmap/badge.svg)](https://docs.rs/cbitmap)

# cbitmap
  
   > A conventional, compact and core (no_std) bitmap.
  
## Use cases

 You are recommended to use this crate when you want to
 maintain a bitmap containing a large but fixed number of bits. 
 Especially, when you are caring memory usage/alignment, for 
 `cbitmap` wastes almost no places.

 For example, you may want to manage a set of resources, which
 can be described by two states, and a bitmap is fit for you.

 If you want to maintain a small set of flags, like 2 or 3, we
 recommend [flagset](https://crates.io/crates/flagset) instead.

 The most extensive and mature implementation of bitmap might be 
 [bitvec](https://crates.io/crates/bitvec). You are recommended
 to use it if you are caring about maturity. 

 Also, [bitset-core](https://crates.io/crates/bitset-core) is 
 another powerful crate that implemented compact bitset trait. 
 However, the implementation between `bitset-core` and `cbitmap` 
 is quiet different. 
 
 What's more, the performance of `cbitmap` has not been tested 
 since it's on alpha version. If you care most about performance, 
 please make a careful consideration before choice.
  
## Features

 We have provided a `crate::bitmap::Bitmap` type:

 ```rust
 pub struct Bitmap<const BYTES: usize> {
     bits: [u8; BYTES],
 }
 ```
  
 You are recommended to use macros to create new bitmaps:

 ```rust
 use cbitmap::bitmap::*;
 
 let map = newmap!(0b_01; 2);
 ```

 See also `crate::he_lang`.

  The bitmap can be manipulated in conventional ways, like
  `Bitmap::test()`,
  `Bitmap::set()`,
  `Bitmap::reset()`,
  `Bitmap::flip()` and
  `Bitmap::at()`.
  Please see the [documentation](https://docs.rs/cbitmap) for 
  detailed examples.
  
  The bitmap is actually a wrapper of `u8` array `[u8; BYTES]`.
  It can be put on diverse places on memory. For example, if 
  the map is relatively small like 8 or 16 bits,
  you can put it on stack safely. If it is larger like 256 or
  1024 bits, you may want to put it on heap.

  The alloc feature which is enabled by default is only used for formatting.
  
## Examples

  Here is a simple example:

  ```rust
  use cbitmap::bitmap::*;
   
  // A macro are provided to create a bitmap.
  let mut map = newmap!(;16);
  
  // There is a set of methods to manipulate the bitmap:
  map.set(10);
  map.reset(10);
  map.flip(10);
  
  // Some C++ like methods are provided:
  assert_eq!(map.test(10), true);
  assert_eq!(map.any(), true);
  assert_eq!(map.none(), false);
  
  // Also provide other useful methods:
  assert_eq!(map.find_first_one(), 10);
 
  // You can access a single bit using wrappers:
  let mut bit = map.at_mut(10);
  assert_eq!(*bit, true);
  bit.flip();
  assert_eq!(*map.at(10), false);
  ```

  Please see the documentation of `Bitmap` and
  the examples dir for detailed examples.
  
  You can use `cargo run --example <name>` to run the examples we
  provide. A simple example is `bitmap-base`, another extensive 
  example about practical usage is `bitmap-usecase`, where bitmap 
  is used to manage raw memory resources.
  
## Current constraints

### Generic const expr

  The bitmap is specified with its size-in-bytes by `BYTES`. This
  is slightly different from conventional `bitset<N>` in C++,
  where `N` indicating the size-in-bits. We implemented bitmap
  in this way to stay on rust-stable, where the
  `#![feature(generic_const_exprs)]` is not supported yet, thus,
  it is not allowed to do like this:

  ```rust
  // requiring #![feature(generic_const_exprs)]
  pub struct Bitmap<const N: usize> {
      bits: [u8; (N + 7) / 8],
  }
  ```

  We have provided an alternative way to let you specify the size
  in bits. The macro `crate::newmap` achieves this:

  ```rust
  const BITS: usize = 16;
  let map = newmap!(;BITS);
  let another = newmap!(;BITS * 2);
  ```

  In principle, it is nevertheless possible to use constexpr when 
  instantiating a struct:

  ```rust
  // allowed:
  let map = Bitmap::<{64 / 8}>::new();
  ```
  
### Index

  A `bitset<N>` in C++ can be indexed by Index op `[]`. We have
  met some problems when implementing this feature. Specifically,
  implementing `core::ops::IndexMut` for a struct is like this:

  ```rust
  impl IndexMut for T {
      type Output = U;
      fn index(&mut self, index: usize) -> &mut Self::Output { ... }
  }
  ```

  The ref in `&mut Self::Output` requires `self` to own the indexed output.
  
  In `Bitmap`, `Output` is required to be "bits".
  It is necessary to use a wrapper type to provide interfaces to
  access a single bits. We have provided `BitRef` and
  `BitRefMut` as the wrappers.
  
  However, the bitmap is not expected to hold a large set of wrappers,
  in order to save memories.
  
  Due to this issue, we only provide  `Bitmap::at_mut()` as methods
  to multably index into the bitmap.

  It is noteworthy that, we provide `Bitmap::at()` to get `BitRef`, and 
  we also provide immutable `Index`. However, immutable `Index` only 
  returns a `bool` value, not `BitRef` due to a similar issue.
 
 ## Updates
 
 - 0.3.2
 - - Add `Index`.
 - - Add `as_ref()`, `as_mut()`, `as_ptr()`, `as_mut_ptr()`.
 - - Wrap general methods like `set()` into a trait.

 - 0.3.1
 - - Add basic benchmarks.
 - - Add new methods compatible with C++: `Bitmap::any()`, 
 `Bitmap::none()`, `Bitmap::all()`, 
 `Bitmap::count()` and `Bitmap::test()`.
 - - Add useful methods: [`Bitmap::find_first_one()`], 
 [`Bitmap::find_first_zero()`].
 - - Add an example `bitmap-usecase`, showing a use case of managing raw
 memory resources with bitmap.
 
 - 0.3.0
 - - Optimize memory usage by removing the `Option` (the size of bitmap 
 is 1 byte bigger than the generic, which is not friendly to memory align).
 - - Add new method: FillPrefix.
 - - Improve creating macros: now length argument receive const exprs.
 
 - 0.2.0
 - - Add building macros.
 
 - 0.1.1
 - - Update docs.
 
 - 0.1.0 
 - - First publish.