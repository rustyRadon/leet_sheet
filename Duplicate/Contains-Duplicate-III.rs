// You are given an integer array nums and two integers indexDiff and valueDiff.

// Find a pair of indices (i, j) such that:

// i != j,
// abs(i - j) <= indexDiff.
// abs(nums[i] - nums[j]) <= valueDiff, and
// Return true if such pair exists or false otherwise.

use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let index_diff = index_diff as usize;
        let mut window = HashMap::with_capacity(index_diff);
        
    }
} 