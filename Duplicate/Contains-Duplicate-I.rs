//Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

impl Sol {
    fn contains_duplicate (mut num: Vec<i32>) -> bool {
        num.sort_unstable

        for i in 1..num.len() {
            if nums[i] == nums[i -1] {
                return true;
            }
        }

        false;
    }
}