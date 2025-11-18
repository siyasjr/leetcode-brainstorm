impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let a: Vec<char> = text1.chars().collect();
        let b: Vec<char> = text2.chars().collect();
        let m = a.len();
        let n = b.len();
        
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if a[i] == b[j] {
                    dp[i][j] = 1 + dp[i + 1][j + 1];
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
                }
            }
        }

        dp[0][0]
    }
}
