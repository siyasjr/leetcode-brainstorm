impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { return 0; }
        if n == 1 { return nums[0]; }

        fn dfs(i: usize, nums: &[i32], memo: &mut Vec<i32>) -> i32 {
            if i >= nums.len() { return 0; }
            if memo[i] != -1 { return memo[i]; }

            let rob = nums[i] + dfs(i + 2, nums, memo);
            let skip = dfs(i + 1, nums, memo);
            memo[i] = rob.max(skip);

            memo[i]
        }

        fn rob_line(nums: &[i32]) -> i32 {
            let mut memo = vec![-1; nums.len()];
            dfs(0, nums, &mut memo)
        }

        let case1 = rob_line(&nums[..n - 1]);
        let case2 = rob_line(&nums[1..]);

        case1.max(case2)
    }
}
