impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        fn dfs(i: usize, j: usize, s: &Vec<char>, p: &Vec<char>, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
            if let Some(val) = memo[i][j] { return val; }

            let ans = if j == p.len() {
                i == s.len()
            } else {
                let first_match = i < s.len() && (p[j] == s[i] || p[j] == '.');

                if j + 1 < p.len() && p[j + 1] == '*' {
                    // skip pattern "*", OR use one match
                    dfs(i, j + 2, s, p, memo) ||
                    (first_match && dfs(i + 1, j, s, p, memo))
                } else {
                    first_match && dfs(i + 1, j + 1, s, p, memo)
                }
            };

            memo[i][j] = Some(ans);
            ans
        }

        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();

        let mut memo = vec![vec![None; p_chars.len() + 1]; s_chars.len() + 1];

        dfs(0, 0, &s_chars, &p_chars, &mut memo)
    }
}

//complexity : O(n·m)