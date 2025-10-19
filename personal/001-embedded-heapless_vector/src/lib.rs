use core::mem::{ManuallyDrop, MaybeUninit};
use core::ptr;

#[derive(Debug)]
pub struct ArrayVec<T, const N: usize> {
    values: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayVec<T, N> {
    /// Creates a new empty ArrayVec
    pub fn new() -> Self {
        // [MaybeUninit<T>; N] is zero-initialized to uninit by default.
        // Meaning the array starts from a blank slate waiting to be
        // initialized by `write()`; filling uninit elements with
        // "garbage" bytes.
        ArrayVec {
            // values: unsafe { MaybeUninit::uninit().assume_init() },
            // Same as the commented code above but safer.
            values: [const { MaybeUninit::uninit() }; N],
            len: 0,
        }
    }

    /// Pushes a value if there's space, returning `Err(value)` if full.
    /// Safe: `.write()` takes ownership and marks the slot as
    /// initialized.
    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.len == N {
            return Err(value);
        }
        self.values[self.len].write(value);
        self.len += 1;
        Ok(())
    }

    /// Returns a reference to the element at `index` if within bounds
    /// and initialized.
    /// SAFETY: Unsafe internally: Assumes first `len` slots are init.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        // Same as the commented code below
        unsafe { Some(&*self.values[index].as_ptr()) }
        // unsafe { Some(self.values[index].assume_init_ref()) }
    }

    /// Pops the last value if any, returning it (or `None` if empty).
    /// SAFETY: Safe: Uses `.assume_init_read()` to extract and mark
    /// as uninit.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(unsafe { self.values[self.len].assume_init_read() })
    }

    /// Returns the current length.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Instead of `into_arr` lets return a slice using
    /// `slice::from_raw_parts()`.
    /// Returns a slice over init elements (& first `len` slots).
    /// SAFETY: Unsafe internally, but safe API: assumes invariant holds.
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.values.as_ptr() as *const T, self.len) }
    }

    /// Returns a mutable slice over init elements.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.values.as_mut_ptr() as *mut T, self.len) }
    }

    /// Use slice iterator for immutable iteration
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    /// Use slice iterator for mutable iteration
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }
}

// Implement Drop trait to safely deallocate init elements.
impl<T, const N: usize> Drop for ArrayVec<T, N> {
    fn drop(&mut self) {
        // Explicity drop the first `len` initialized elements.
        // SAFETY: Will not drop uninit values
        for i in 0..self.len {
            unsafe {
                self.values[i].assume_init_drop();
            }
        }
    }
}

// Build out iterator type for ArrayVec as ArrayVecIntoIter<T, N>
// Consuming iterator (by-value): Moves out owned T.
// But will provide iterators for fundamental types: & and &mut
#[derive(Debug)]
pub struct ArrayVecIntoIter<T, const N: usize> {
    values: [MaybeUninit<T>; N],
    len: usize,
    index: usize,
}

// Implement Iterator trait on ArrayVecIntoIter making it an iterator.
impl<T, const N: usize> Iterator for ArrayVecIntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        let i = self.index;
        self.index += 1;
        // SAFETY: i < len, so slot is init
        Some(unsafe { self.values[i].assume_init_read() })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

// Implement Drop trait to safely deallocate initialized elements
// from ArrayVecIntoIter<T, N> MaybeUninit<T> values
impl<T, const N: usize> Drop for ArrayVecIntoIter<T, N> {
    fn drop(&mut self) {
        // Drop remaining init elements (from index to len)

        // SAFETY: Just for extra safety:
        #[cfg(debug_assertions)]
        debug_assert!(self.index <= self.len);

        // SAFETY: Invariant holds for those slots.
        for i in self.index..self.len {
            unsafe {
                self.values[i].assume_init_drop();
            }
        }
        // Uninit slots auto-drop as MaybeUninit: meaning as "garbage"
        // bytes they are overwritten by the next write.
    }
}

// Implement IntoIterator for ArrayVecIntoIter; returns an iterator.
impl<T, const N: usize> IntoIterator for ArrayVec<T, N> {
    type Item = T;
    type IntoIter = ArrayVecIntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: Wrap in ManuallyDrop to prevent original drop from running.
        // This transfers control to the iterator's Drop.
        let this = ManuallyDrop::new(self);
        // SAFETY: Read fields out (valid as long as we don't access `this` after).
        // ptr::read performs bitwise copy without calling Drop on read memory
        let values = unsafe { ptr::read(&this.values) };
        let len = unsafe { ptr::read(&this.len) };

        ArrayVecIntoIter {
            values,
            len,
            index: 0,
        }
    }
}

// Implement IntoIterator for fundamental types of ArrayVec: & and &mut.
// By reference: yields &T
impl<'a, T, const N: usize> IntoIterator for &'a ArrayVec<T, N> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

// By mutable reference: yields &mut T
impl<'a, T, const N: usize> IntoIterator for &'a mut ArrayVec<T, N> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}
