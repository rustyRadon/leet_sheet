//Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.

use std::collections::HashSet;

impl Sol{
    pub fun contains_nearby_duplicate(nums: Vec <i32>, k: i32) -> bool {
        let k = k as usize;
        let mut window = HashSet::with_capacity(k + 1);

        for i in 0..nums.len() {
            if i > k {
                window.remove(&nums[i- k- 1]);
            }

            if !window.insert(nums[i]) {
                return true;
            }
        }

        false;
    }
}