// You are given an integer array nums and two integers indexDiff and valueDiff.

// Find a pair of indices (i, j) such that:

// i != j,
// abs(i - j) <= indexDiff.
// abs(nums[i] - nums[j]) <= valueDiff, and
// Return true if such pair exists or false otherwise.
// 
// BUCKET MAPPP


use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        k: i32,  //The maximum distance in line allowed between two numbers.
        t: i32,  //The maximum difference allowed between two numbers.
    ) -> bool {
        if k < 0 || t < 0 { return false; }
        
        let k = k as usize;  //usize for counting positions/indices in a list.
        let t = t as i64;    //i64 (a much bigger integer) so if we do math with giant numbers, they won't overflow and break the computer.
        let width = t + 1;              // bucket size
        let mut buckets = HashMap::<i64, i64>::with_capacity(k.min(nums.len()) + 1);
        
        for (i, &num) in nums.iter().enumerate() {
            let num = num as i64;
            let bucket = num.div_euclid(width);  //calculates which bucket a number belongs to.
            
            // check same bucket
            if let Some(_) = buckets.get(&bucket) {
                return true;
            }
            
            // check left neighbor bucket
            if let Some(&v) = buckets.get(&(bucket - 1)) {
                if num - v <= t {
                    return true;
                }
            }
            
            // check right neighbor bucket
            if let Some(&v) = buckets.get(&(bucket + 1)) {
                if v - num <= t {
                    return true;
                }
            }
            
            // insert current
            buckets.insert(bucket, num);
            
            // Remove element that falls out of the window (i >= k)
            if i >= k {
                let old = nums[i - k] as i64;
                let old_bucket = old.div_euclid(width);
                buckets.remove(&old_bucket);
            }
        }
        
        false
    }
}