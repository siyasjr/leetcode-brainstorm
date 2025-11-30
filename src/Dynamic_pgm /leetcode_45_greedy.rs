impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![i32::MAX; n];
        dp[0] = 0;

        for i in 0..n {
            let max_jump = nums[i] as usize;
            for j in i+1..=i + max_jump {
                if j < n {
                    dp[j] = dp[j].min(dp[i] + 1);
                }
            }
        }

        dp[n - 1]
    }
}

//complexity: O(n²)