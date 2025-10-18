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
}
