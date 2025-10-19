// Creating a heapless vector type using MaybeUninit<T> safely for
// unintialized memory (useful in embedded systems where there's limited
// memory space for heap allocations).
#![no_std]

// Only using stdlib to print to stdout & stderr for debugging
extern crate std;

use heapless_vector::ArrayVec;

const CAP: usize = 5;

fn main() {
    {
        // A:
        let mut arr_vec = ArrayVec::<i32, CAP>::new();
        std::println!("{:?}", arr_vec);

        let mut count;

        // Push a few elements
        for i in 0..(CAP - 2) {
            count = 1 + i as i32;
            arr_vec.try_push(count).unwrap();
        }
        std::println!("{:?}", arr_vec);

        // Return element in initialized index;
        // if index is not initialized, return None.
        let arr_els = arr_vec.as_slice();
        std::println!("---\nInit values: {:?}", arr_els);

        // Pop from MaybeUninit ArrayVec; if uninit, return None.
        std::println!("---\nArrayVec `len` before pop: {}", arr_vec.len());
        std::println!("Popped value: {:?}", arr_vec.pop());
        std::println!("ArrayVec `len` after pop: {}", arr_vec.len());

        // TEST: Add more elements beyond `CAP` size for ArrayVec;
        // `try_push` should escape and return with Err.
        let arr_len = arr_vec.len();
        count = *arr_vec.get(arr_len - 1).unwrap();
        let mut arr_err_els: ArrayVec<Result<(), i32>, CAP> = ArrayVec::new();

        for _ in arr_len..(CAP * 2) {
            count += 1;
            if let Err(value) = arr_vec.try_push(count) {
                arr_err_els.try_push(Err(value)).unwrap();
            }
        }
        std::println!("---\nFilled ArrayVec: {:?}", arr_vec.as_slice());
        std::println!("Err beyond ArrayVec: {:?}", arr_err_els.as_slice());
    }

    {
        // B:
        // Iterating over ArrayVec through fundamental types: & and &mut

        // Simple iteration
        let mut arr_vec = ArrayVec::<u8, CAP>::new();
        let mut count;
        for i in 0..CAP {
            count = 1 + i as u8;
            arr_vec.try_push(count).unwrap(); // Init values on ArrayVec
        }
        std::println!("---");
        count = 0;
        for i in &arr_vec {
            std::println!("ArrayVec at index {}: {}", count, i);
            count += 1;
        }

        // Mutable iteration
        for value in &mut arr_vec {
            *value += 10;
        }
        std::println!("---\nMutated ArrayVec: {:?}", arr_vec.as_slice());

        // Problem: iterate over non-iterable (owned) ArrayVec

        /*
        This problem occurs because (owned) ArrayVec does not implement IntoIterator.
        Ultimately, I believe it's better this way since in embedded environments we
        try to manage memory space, therefore there's no need to consume and replicate
        data unneccesarily.
        */
        // for i in arr_vec {
        //     unreachable!();
        // }
    }

    {
        // C:
        // Here, I've implemented the ability to iterate over owned ArrayVec.
        // Iterating over owned values of ArrayVec by calling `into_iter`
        let mut arr_vec = ArrayVec::<u8, CAP>::new();
        let mut count;
        for i in 0..(CAP - 1) {
            count = 1 + i as u8;
            arr_vec.try_push(count).unwrap();
        }
        let mut arr_iter = arr_vec.into_iter(); // move `arr_vec`.
        // std::println!("{:?}", arr_vec); // should return move error
        // Above code confirms `impl IntoIterator for ArrayVec` safety invariants.

        std::println!("---\n{:?}", arr_iter); /* View created ArrayVecIntoIter.
        See `len` and `index` fields. */

        let mut arr_vec2: ArrayVec<Option<u8>, CAP> = ArrayVec::new();
        loop {
            /* Loop through calls to `next()` to iterate through
            ArrayVecIntoIter. */
            match arr_iter.next() {
                Some(mut value) => {
                    value += 10;
                    arr_vec2.try_push(Some(value)).unwrap();
                }
                None => {
                    arr_vec2.try_push(None).unwrap();
                    break;
                }
            }
        }
        std::println!("{:?}", arr_iter); /* Review `len` and `index` fields.
        Notice index incr caused by calling `next()` on ArrayVecIntoIter. */
        std::println!("{:?}", arr_vec2.as_slice());
    }

    {
        // D:
        // Test FromIterator implementation with iterators on ArrayVec.

        // Using `collect`:
        let arr_vec = (-10..10).collect::<ArrayVec<i8, 15>>();
        std::println!("---\n{:?}", arr_vec.as_slice());

        // Using `from_iter`:
        let arr_vec = ArrayVec::<_, 5>::from_iter(-3..5 as i8); /* Using
        type inference for `T` in `ArrayVec<T, N>` helped by type cast
        on iterator. */
        std::println!("{:?}", arr_vec.as_slice());
    }

    {
        // E:
        // Test Extend implementation with iterators on ArrayVec.

        /* Testing for two scenarios;

        1. `arr_vec1` has more CAP (array "capacity" - N) than `arr_vec2`
        has elements to fill it. Meaning, `arr_vec1` will retain spare CAP.

        2. `arr_vec1` has less CAP (array "capacity" - N) than `arr_vec2`
        has elements to fill it. Meaning, `arr_vec1` will max it's CAP
        without extending all of `arr_vec2`s elements.
        */

        // Scenario 1:
        type ArrayVecCap10<T> = ArrayVec<T, 10>;
        type ArrayVecCap5<T> = ArrayVec<T, 5>;

        let mut arr_vec1: ArrayVecCap10<u8> = ArrayVec::new();
        let mut count;
        for i in 0..3 {
            // Add elements to `arr_vec1`.
            count = 1 + i as u8;
            arr_vec1.try_push(count).unwrap();
        }

        let mut arr_vec2: ArrayVecCap5<u8> = ArrayVec::new();
        for i in 0..2 {
            // Add elements to `arr_vec2`.
            count = 1 + i as u8;
            arr_vec2.try_push(count).unwrap();
        }

        arr_vec1.extend(arr_vec2);

        let mut empty_arr_vec: ArrayVecCap10<Option<u8>> = ArrayVec::new();
        std::println!("{:?}", arr_vec1.show_init(&mut empty_arr_vec));
        drop(empty_arr_vec); /* Choosing to explicity drop (or free, or
        deallocate) the stack memory consumed by `empty_arr_vec` here
        as it's no longer in use. */

        // Scenario 2:
        let mut arr_vec1: ArrayVecCap5<i8> = ArrayVec::new();
        for i in -2..0 {
            arr_vec1.try_push(i).unwrap();
        }

        let mut arr_vec2: ArrayVecCap10<i8> = ArrayVec::new();
        for i in -5..0 {
            arr_vec2.try_push(i).unwrap();
        }

        arr_vec1.extend(arr_vec2);

        let mut empty_arr_vec: ArrayVecCap5<Option<i8>> = ArrayVec::new();
        std::println!("{:?}", arr_vec1.show_init(&mut empty_arr_vec));
        /* Don't need to explicity drop `empty_arr_vec` here as before
        because this is the end of the scope, and the Rust Compiler (rustc)
        will call the drop method in ArrayVec's destructor `Drop`
        to drop `empty_arr_vec` implicity. */
    }
}
