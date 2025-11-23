impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut next1 = 1; // dp[n]
        let mut next2 = 0; // dp[n+1] (dummy)

        for i in (0..n).rev() {
            let mut cur = 0;

            if bytes[i] != b'0' {
                cur = next1;

                if i + 1 < n {
                    let val = (bytes[i] - b'0') * 10 + (bytes[i + 1] - b'0');
                    if val <= 26 {
                        cur += next2;
                    }
                }
            }

            next2 = next1;
            next1 = cur;
        }

        next1
    }
}

//complexity : O(1)