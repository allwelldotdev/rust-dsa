//! # Valid Parentheses algorithm
//!
//! Given a string s containing '(', ')', '{', '}', '[', ']', determine if the input
//! string is valid. Open brackets must be closed by the same type in correct order.

use valid_parentheses::Solution;

fn main() {
    let s = String::from("{[]}(){}[]");
    let sol = Solution::is_valid(s);
    println!("{sol:?}");
}
