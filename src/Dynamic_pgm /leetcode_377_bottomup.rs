impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; (target + 1) as usize];
        dp[0] = 1;

        for t in 1..=target as usize {
            for &n in &nums {
                if n as usize <= t {
                    dp[t] += dp[t - n as usize];
                }
            }
        }

        dp[target as usize]
    }
}

//complexity: O(n * target)**