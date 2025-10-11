//! # Two Sum algorithm
//!
//! Given an array of integers `nums` and an integer `target`, return indices of the
//! two numbers such that they add up to `target`. Assume exactly one solution, return
//! in any order.

use two_sum::Solution;

fn main() {
    let num_vec = vec![2, 4, 6, 8, 10];
    let num_target = 18;
    let sol = Solution::two_sums(num_vec, num_target);
    println!("{:?}", sol);
}
