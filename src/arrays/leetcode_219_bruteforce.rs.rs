impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        for i in 0..n {
            for j in i+1..=(i + k as usize).min(n - 1) {
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }
        false
    }
}

// complexity : O(n·k)