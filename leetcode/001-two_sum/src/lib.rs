//! # Two Sum algorithm (solution explained)
//!
//! 1. Use hashmap to store seen numbers and indices.
//! 2. For each `num`, check if `target - num` exists in the map (O(1) "constant time" lookup).
//!     If yes, return indices; else, insert. This avoids O(n^2) brute force.
//!
//! NOTES: `enumerate()` for indices; `as i32` for type casting. Handles ownership by
//! borrowing `nums`.
//!
//! Time: O(n), Space: O(n).
//! Edge cases: Negative numbers, duplicates.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sums(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&val) = map.get(&complement) {
                return vec![val, i as i32];
            }
            map.insert(num, i as i32);
        }
        vec![]
    }
}
