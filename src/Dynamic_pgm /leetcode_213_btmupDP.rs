impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { return 0; }
        if n == 1 { return nums[0]; }

        fn rob_line(nums: &[i32]) -> i32 {
            let m = nums.len();
            if m == 0 { return 0; }
            if m == 1 { return nums[0]; }

            let mut dp = vec![0; m];
            dp[0] = nums[0];
            dp[1] = nums[0].max(nums[1]);

            for i in 2..m {
                dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
            }
            dp[m - 1]
        }

        let case1 = rob_line(&nums[..n - 1]);
        let case2 = rob_line(&nums[1..]);

        case1.max(case2)
    }
}
