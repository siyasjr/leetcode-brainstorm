impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                return nums[i];
            }
        }
        nums[0]
    }
}

// Complexity : O(k), where k is the rotation amount | Not guaranteed O(log n), but faster in small rotations.
