impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn dfs(i: usize, nums: &Vec<i32>) -> i32 {
            if i >= nums.len() {
                return 0;
            }
            let rob = nums[i] + dfs(i + 2, nums);
            let skip = dfs(i + 1, nums);
            rob.max(skip)
        }

        dfs(0, &nums)
    }
}
