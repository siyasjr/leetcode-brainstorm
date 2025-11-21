impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn dfs(i: usize, nums: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
            if i >= nums.len() { return 0; }

            if memo[i] != -1 {
                return memo[i];
            }

            let rob = nums[i] + dfs(i + 2, nums, memo);
            let skip = dfs(i + 1, nums, memo);
            memo[i] = rob.max(skip);

            memo[i]
        }

        let mut memo = vec![-1; nums.len()];
        dfs(0, &nums, &mut memo)
    }
}
