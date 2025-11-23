impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        fn dfs(i: usize, s: &str) -> i32 {
            if i == s.len() { return 1; }
            if &s[i..i+1] == "0" { return 0; }

            let mut ways = dfs(i + 1, s); // single digit

            // two digits
            if i + 2 <= s.len() && s[i..i+2].parse::<i32>().unwrap() <= 26 {
                ways += dfs(i + 2, s);
            }
            ways
        }

        dfs(0, &s)
    }
}

// complexity : O(2ⁿ)
