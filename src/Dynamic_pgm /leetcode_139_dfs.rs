use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let set: HashSet<String> = word_dict.into_iter().collect();
        let mut memo = vec![None; s.len() + 1];

        fn dfs(i: usize, s: &str, set: &HashSet<String>, memo: &mut Vec<Option<bool>>) -> bool {
            if i == s.len() { return true; }
            if let Some(v) = memo[i] { return v; }

            for j in i+1..=s.len() {
                if set.contains(&s[i..j]) && dfs(j, s, set, memo) {
                    memo[i] = Some(true);
                    return true;
                }
            }
            memo[i] = Some(false);
            false
        }

        dfs(0, &s, &set, &mut memo)
    }
}

//complexity : O(n²)