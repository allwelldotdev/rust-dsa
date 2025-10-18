// Creating a heapless vector type using MaybeUninit<T> safely for
// unintialized memory (useful in embedded systems where there's limited
// memory space for heap allocations).
#![no_std]

// Only using stdlib to print to stdout & stderr for debugging
extern crate std;

use heapless_vector::ArrayVec;

const CAP: usize = 5;

fn main() {
    let mut arr_vec1 = ArrayVec::<i32, CAP>::new();
    std::println!("{:?}", arr_vec1);

    let mut count;

    // Push a few elements
    for i in 0..(CAP - 2) {
        count = 1 + i as i32;
        arr_vec1.try_push(count).unwrap();
    }
    std::println!("{:?}", arr_vec1);

    // Return element in initialized index;
    // if index is not initialized, return None.
    let arr_els1 = arr_vec1.as_slice();
    std::println!("---\nInit values: {:?}", arr_els1);

    // Pop from MaybeUninit ArrayVec; if uninit, return None.
    std::println!("---\nArrayVec `len` before pop: {}", arr_vec1.len());
    std::println!("Popped value: {:?}", arr_vec1.pop());
    std::println!("ArrayVec `len` after pop: {}", arr_vec1.len());

    // TEST: Add more elements beyond `CAP` size for ArrayVec;
    // `try_push` should escape and return with Err.
    let arr_len = arr_vec1.len();
    count = *arr_vec1.get(arr_len - 1).unwrap();
    let mut arr_err_els: ArrayVec<Result<(), i32>, CAP> = ArrayVec::new();

    for _ in arr_len..(CAP * 2) {
        count += 1;
        if let Err(value) = arr_vec1.try_push(count) {
            arr_err_els.try_push(Err(value)).unwrap();
        }
    }
    std::println!("---\nFilled ArrayVec: {:?}", arr_vec1.as_slice());
    std::println!("Err beyond ArrayVec: {:?}", arr_err_els.as_slice());
}
