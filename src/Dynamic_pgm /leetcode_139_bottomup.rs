use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let set: HashSet<String> = word_dict.into_iter().collect();
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 0..n {
            if !dp[i] { continue; }
            for j in i+1..=n {
                if set.contains(&s[i..j]) {
                    dp[j] = true;
                }
            }
        }
        dp[n]
    }
}
