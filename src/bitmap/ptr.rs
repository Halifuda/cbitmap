use super::*;

impl<const BYTES: usize> Bitmap<BYTES> {
  pub fn as_ref(&self) -> &[u8; BYTES] {
    &self.bits
  }

  pub fn as_mut(&mut self) -> &mut [u8; BYTES] {
    &mut self.bits
  }

  pub fn as_ptr(&self) -> *const u8 {
    self.bits.as_ptr()
  }

  pub fn as_mut_ptr(&mut self) -> *mut u8 {
    self.bits.as_mut_ptr()
  }
}