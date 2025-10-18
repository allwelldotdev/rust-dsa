use core::mem::MaybeUninit;

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

    /// Return array of init and uninit elements in ArrayVec.
    ///
    /// ASIDE: I don't actually need to add `'a` due to Lifetime Ellision
    /// rules. I left it just to remind myself how far I've come to
    /// perfectly understand lifetimes now.
    ///
    /// Use `into_arr` as an associated function.
    // pub fn into_arr<'a>(arr_vec: &'a ArrayVec<T, N>)
    // -> [Option<&'a T>; N]
    // {
    //     let mut arr: [Option<&T>; N] = [const { None }; N];
    //     for i in 0..N {
    //         let el = arr_vec.get(i);
    //         arr[i] = el;
    //     }
    //     arr
    // }

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

    // /// Returns a mutable slice over init elements.
    // pub fn as_mut_slice(&mut self) -> &mut [T] {
    //     unsafe { core::slice::from_raw_parts_mut(self.values.as_mut_ptr() as *mut T, self.len) }
    // }

    // /// Use slice iterator for immutable iteration
    // pub fn iter(&self) -> core::slice::Iter<'_, T> {
    //     self.as_slice().iter()
    // }

    // /// Use slice iterator for mutable iteration
    // pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
    //     self.as_mut_slice().iter_mut()
    // }
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
