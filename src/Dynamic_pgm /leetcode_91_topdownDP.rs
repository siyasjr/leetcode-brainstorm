impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut memo = vec![-1; s.len() + 1];

        fn dfs(i: usize, s: &str, memo: &mut Vec<i32>) -> i32 {
            if i == s.len() { return 1; }
            if s.as_bytes()[i] == b'0' { return 0; }
            if memo[i] != -1 { return memo[i]; }

            let mut ways = dfs(i + 1, s, memo);

            if i + 1 < s.len() {
                let val = (s.as_bytes()[i] - b'0') * 10 + (s.as_bytes()[i + 1] - b'0');
                if val <= 26 {
                    ways += dfs(i + 2, s, memo);
                }
            }

            memo[i] = ways;
            ways
        }

        dfs(0, &s, &mut memo)
    }
}

//complexity : O(n)