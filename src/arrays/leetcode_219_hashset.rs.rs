use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut set = HashSet::new();

        for i in 0..nums.len() {
            if set.contains(&nums[i]) {
                return true;
            }
            set.insert(nums[i]);
            if set.len() > k as usize {
                set.remove(&nums[i - k as usize]);
            }
        }
        false
    }
}





