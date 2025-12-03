impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        let n = s.len();
        let m = p.len();

        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[0][0] = true;

        // pattern like a*, a*b*, a*b*c* ...
        for j in 1..=m {
            if p[j - 1] == '*' && j >= 2 {
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=n {
            for j in 1..=m {
                if p[j - 1] == '*' {
                    dp[i][j] =
                        dp[i][j - 2] // zero occurrences
                        || ((p[j - 2] == '.' || p[j - 2] == s[i - 1]) && dp[i - 1][j]); // one or more
                } else {
                    dp[i][j] =
                        (p[j - 1] == '.' || p[j - 1] == s[i - 1]) &&
                        dp[i - 1][j - 1];
                }
            }
        }

        dp[n][m]
    }
}
//complexity : O(n·m)