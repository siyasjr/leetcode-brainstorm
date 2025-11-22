impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn dfs(i: usize, nums: &[i32]) -> i32 {
            if i >= nums.len() { return 0; }
            let rob = nums[i] + dfs(i + 2, nums);
            let skip = dfs(i + 1, nums);
            rob.max(skip)
        }

        let n = nums.len();
        if n == 0 { return 0; }
        if n == 1 { return nums[0]; }

        dfs(0, &nums[..n - 1]).max(dfs(0, &nums[1..]))
    }
}
