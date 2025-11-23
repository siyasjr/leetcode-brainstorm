impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut dp = vec![0; n + 1];

        dp[n] = 1; // empty string = 1 way

        for i in (0..n).rev() {
            if bytes[i] == b'0' {
                dp[i] = 0;
            } else {
                dp[i] = dp[i + 1];

                if i + 1 < n {
                    let val = (bytes[i] - b'0') * 10 + (bytes[i + 1] - b'0');
                    if val <= 26 {
                        dp[i] += dp[i + 2];
                    }
                }
            }
        }
        
        dp[0]
    }
}

//complexity : O(n)