//! # Merge Two Sorted Lists
//!
//! Merge two sorted linked lists and return as a sorted list. Splice nodes together.

use merge_sorted_lists::{ListNode, Solution};

fn main() {
    let (node1, node2) = (Box::new(ListNode::new(1)), Box::new(ListNode::new(2)));
    let sol = Solution::merge_two_lists(Some(node1), Some(node2));
    println!("{sol:?}");
}
