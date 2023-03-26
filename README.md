[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# cbitmap
  
   > A conventional, compact and core (no_std) bitmap.
  
  ## Use cases
  You are recommended to use this crate when you want to 
  maintain a bitmap containing a large but fixed number of bits. 
  
  For example, you may want to manage a set of resources, which 
  can be described by two states, and a bitmap is fit for you. 
  
  If you want to maintain a small set of flags, like 2 or 3, we
  recommend [flagset](https://crates.io/crates/flagset) instead.
  
  ## Features
  We provided a `crate::bitmap::Bitmap` type:
  ```rust
  pub struct Bitmap<const BYTES: usize> {
      bits: Option<[u8; BYTES]>,
  }
  ```
  
  The bitmap can be manipulated in conventional ways, like 
  `crate::bitmap::Bitmap::get_bool()`, 
  `crate::bitmap::Bitmap::set()`, 
  `crate::bitmap::Bitmap::reset()`, 
  `crate::bitmap::Bitmap::flip()` and 
  `crate::bitmap::Bitmap::at()`.
  
  The bitmap is actually a wrapper of `u8` array `[u8; BYTES]`.
  It can be put on diverse places on memory. 
  
  For example, if the map is relatively small like 8 or 16 bits, 
  you can put it on stack safely. If it is larger like 256 or 
  1024 bits, you may want to put it on heap. 
  
  ## Examples
  Please see the documentation of `crate::bitmap::Bitmap` and 
  the examples dir.
  
  You can use `cargo run --example <name>` to run the examples we 
  provide.
  
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
  and we failed to find a way to workaround this problem. 
  
  ### Index
  A `bitset<N>` in C++ can be indexed by Index op `[]`. We have 
  meet some problems when implementing this feature. Specifically, 
  implementing `core::ops::Index` for a struct is like this:
  ```rust
  impl Index for T {
      type Output = U;
      fn index(&self, index: usize) -> &Self::Output { ... }
  }
  ```
  The ref in `&Self::Output` requires `self` to own the indexed output. 
  
  In `crate::bitmap::Bitmap`, `Output` is required to be "bits". 
  It is necessary to use a wrapper type to provide interfaces to 
  access a single bits. We provided `crate::bitmap::BitRef` and 
  `crate::bitmap::BitRefMut` as the wrappers. 
  
  However, the bitmap is not expected to hold a large set of wrappers 
  in order to save memories. 
  It is not possible either to create the wrapper in `index()` and 
  pass it to the bitmap, since the `self` is referenced immutably.
  
  Due to this issue, we only provide `crate::bitmap::Bitmap::at()` 
  and `crate::bitmap::Bitmap::at_mut()` as methods
  to index into the bitmap.