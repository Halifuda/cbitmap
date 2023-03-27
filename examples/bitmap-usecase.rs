//! Here, we implemented a simple cacheline manager, 
//! which manages the allocating and deallocating of
//! 64B cachelines in a single 4K page. With the help
//! of Bitmap, it is possible to use only 1/64 of total
//! memory to maintain the allocation status. 

extern crate alloc;
extern crate cbitmap;

use cbitmap::bitmap::*;

type Cacheline = [u8; 64];
type Page = [u8; 4096];

/// Must be `repr(C)`, making the bitmap's address fixed.
#[repr(C)]
struct CachelineManager<'a> {
    _bitmap: &'a Bitmap<8>,
    page: &'a mut Page,
}

#[derive(Debug)]
enum ManagerError {
    Oor(i128),
    Oom,
    Realloc,
    Unallocated,
    Unknown,
}

use ManagerError::*;

impl<'a> CachelineManager<'a> {
    /// Count of usable cacheline.
    ///
    /// As bitmap occupies 1 line, only 63.
    const LINECNT: usize = 63;

    /// Get the ptr of bitmap.
    unsafe fn get_map_ptr(&self) -> core::ptr::NonNull<Bitmap<8>> {
        let raw = self.page.as_ptr().cast::<Bitmap<8>>();
        raw.as_ref().unwrap().into()
    }

    /// Get the ptr of line array.
    unsafe fn get_lines_start_ptr(&self) -> core::ptr::NonNull<Cacheline> {
        let raw = self.page.as_ptr().cast::<Cacheline>().add(1);
        raw.as_ref().unwrap().into()
    }

    /// Create a manager with a pre allocated `Page`.
    fn new(page: &'a mut Page) -> Self {
        // set the first bit to '1', for bitmap has occupied it.
        let bitmap = unsafe {
            let mapptr = page.as_mut_ptr().cast::<Bitmap<8>>();
            *mapptr = newmap!(0b1; 64);
            mapptr.as_ref().unwrap()
        };
        println!("new {}", bitmap.find_first_zero().unwrap());
        Self {
            _bitmap: bitmap,
            page,
        }
    }

    /// Allocate a mutable cacheline.
    ///
    /// # Fails on:
    /// - `Oom`: Out or memory, indicating all the cachelines in the
    /// manager are allocated.
    /// - `Reallocate`: Allocating at a cacheline that has already been
    /// allocated. This is not expected to happen if the implementation
    /// of `Bitmap` is correct.
    /// - `Unknown`: Specifically, when the cacheline is not allocated
    /// but the calculated pointer happens to be `Null`. This is not
    /// expected to happen.
    fn allocate(&self) -> Result<&mut Cacheline, ManagerError> {
        unsafe {
            let map = self.get_map_ptr().as_mut();
            let start = self.get_lines_start_ptr().as_ptr();
            // Cannot find a free line.
            let idx = map.find_first_zero().ok_or(Oom)?;
            match map.test(idx) {
                // Has been allocated.
                true => Err(Realloc),
                false => {
                    map.set(idx);
                    // Sutract 1 for start is actually at 1
                    start.add(idx - 1).as_mut().ok_or(Unknown)
                }
            }
        }
    }

    /// Deallocate a existing cacheline.
    ///
    /// # Fails on:
    /// - `Unallocated`: Deallocating a cacheline that hasn't been
    /// allocated previously.
    fn deallocate(&self, line: &mut Cacheline) -> Result<(), ManagerError> {
        let idx = self.get_idx(line)?;
        unsafe {
            let map = self.get_map_ptr().as_mut();
            if !map.test(idx) {
                Err(Unallocated)
            } else {
                map.reset(idx);
                Ok(())
            }
        }
    }

    /// Get the cacheline index in the page, it will not be 0, for the
    /// 0-cacheline is occupied by bitmap.
    ///
    /// # Fails on:
    /// - `Oor(i128)`: Out of range. The `i128` is the raw index, which
    /// is out of a page's range.
    fn get_idx(&self, line: &Cacheline) -> Result<usize, ManagerError> {
        let page = self.page.as_ptr() as usize;
        let ptr = line.as_ptr() as usize;
        if ptr < page {
            Err(Oor(
                ((ptr as i128) - (page as i128)) / (core::mem::size_of::<Cacheline>() as i128)
            ))
        } else {
            let len = ptr - page;
            let idx = len / core::mem::size_of::<Cacheline>();
            match idx > Self::LINECNT {
                true => Err(Oor(idx as i128)),
                false => Ok(idx),
            }
        }
    }
}

fn main() {
    let mut page = Box::new([0u8; 4096]);
    let manager = CachelineManager::new(page.as_mut());
    let mut lines = vec![];

    println!("Allocating 8 cachelines, their indexes:");
    for _ in 0..8 {
        lines.push(manager.allocate().unwrap());
    }
    for line in &mut lines {
        let idx = manager.get_idx(line).unwrap() as u8;
        println!("idx = {idx}");
        line[0] = idx;
    }

    println!("Deallocating cacheline 7.");
    // Subtract 1 for bitmap occupied a cacheline, idx starts at 1.
    let line7 = lines.remove(7 - 1);
    manager.deallocate(line7).unwrap();
    println!("Reallocating a cacheline, its idx should be 7.");
    let newline = manager.allocate().unwrap();
    assert_eq!(manager.get_idx(newline).unwrap(), 7);
    lines.push(newline);

    for _ in 8..63 {
        lines.push(manager.allocate().unwrap());
    }
    println!("Using up the page: #lines = {}.", lines.len());
    let err = manager.allocate();
    assert!(err.is_err());
    println!("Allocate another cacheline but failed: {err:?}");

    let (err, ptr) = unsafe {
        let line = lines.pop().unwrap();
        let rawptr = line.as_mut_ptr().cast::<Cacheline>();
        manager.deallocate(line).unwrap();
        (manager.deallocate(rawptr.as_mut().unwrap()), rawptr)
    };
    assert!(err.is_err());
    println!("Deallocate a cacheline twice but failed: {err:?}");
    assert_eq!(lines.len(), 62);

    let (err, ptr) = unsafe {
        (manager.deallocate(ptr.add(1).as_mut().unwrap()), ptr)
    };
    assert!(err.is_err());
    println!("Deallocate a cacheline at index 64 but failed: {err:?}");

    let (err, _ptr) = unsafe {
        (manager.deallocate(ptr.sub(64).as_mut().unwrap()), ptr)
    };
    assert!(err.is_err());
    println!("Deallocate a cacheline at index -1 but failed: {err:?}");
}
