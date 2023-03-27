/// Some useful tools for bitmap implementation. Not exposed.
pub(super) mod inner_use {
    #[inline]
    pub(crate) fn __byte_or_u8(byte: &mut u8, mask: u8) {
        *byte |= mask;
    }

    #[inline]
    pub(crate) fn __byte_and_u8(byte: &mut u8, mask: u8) {
        *byte &= mask;
    }

    #[inline]
    pub(crate) fn __idx_get_byte(index: usize) -> usize {
        index >> 3
    }

    #[inline]
    pub(crate) fn __idx_get_bit(index: usize) -> usize {
        index & 0b111
    }

    #[inline]
    pub(crate) fn __idx_1dto2d(index: usize) -> (usize, usize) {
        (__idx_get_byte(index), __idx_get_bit(index))
    }

    #[inline]
    pub(crate) fn __out_bound(bytes: usize, index: usize) -> bool {
        __idx_get_byte(index) >= bytes
    }

    #[inline]
    pub(crate) fn __copy_bytes<const N: usize, const M: usize>(src: [u8; M]) -> [u8; N] {
        let mut dst = [0u8; N];
        unsafe {
            match N > M {
                true => {
                    let dstptr = dst.as_mut_ptr().cast::<[u8; M]>();
                    core::ptr::write(dstptr, src);
                }
                false => {
                    let dstptr = dst.as_mut_ptr().cast::<[u8; N]>();
                    let srcptr = src.as_ptr().cast::<[u8; N]>();
                    core::ptr::write(dstptr, *srcptr);
                }
            }
        };
        dst
    }

    #[inline]
    pub(crate) fn __copy_bytes_to<const N: usize, const M: usize>(
        dst: &mut [u8; N],
        src: &mut [u8; M],
    ) {
        unsafe {
            match N > M {
                true => {
                    let dstptr = dst.as_mut_ptr().cast::<[u8; M]>();
                    core::ptr::write(dstptr, *src);
                }
                false => {
                    let dstptr = dst.as_mut_ptr().cast::<[u8; N]>();
                    let srcptr = src.as_ptr().cast::<[u8; N]>();
                    core::ptr::write(dstptr, *srcptr);
                }
            }
        };
    }
}