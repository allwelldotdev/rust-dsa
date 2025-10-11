//! # Merge Two Sorted Lists
//!
//! 1. Recursively compare heads, attaching smaller to result and recursing on tails.
//! 2. Base: If one list empty, return other.
//!
//! NOTES: Pattern matching on Options; Box for heap allocation.
//!
//! Time: O(n + m), Space: O(n + m) (stack).
//! Iterative alternative uses dummy node for O(1) space.

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(node), None) => Some(node),
            (None, Some(node)) => Some(node),
            (Some(node1), Some(node2)) => {
                if node1.val <= node2.val {
                    Some(Box::new(ListNode {
                        val: node1.val,
                        next: Self::merge_two_lists(node1.next, Some(node2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: node2.val,
                        next: Self::merge_two_lists(Some(node1), node2.next),
                    }))
                }
            }
        }
    }
}
