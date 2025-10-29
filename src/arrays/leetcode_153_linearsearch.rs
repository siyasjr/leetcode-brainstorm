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
