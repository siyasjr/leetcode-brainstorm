
impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let mut dp = vec![0; (target + 1) as usize];
        dp[0] = 1;

        for t in 1..=target as usize {
            for &n in &nums {
                let v = n as usize;
                if v > t { break; }
                dp[t] += dp[t - v];
            }
        }

        dp[target as usize]
    }
}
