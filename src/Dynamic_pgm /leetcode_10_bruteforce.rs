impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        fn dfs(i: usize, j: usize, s: &Vec<char>, p: &Vec<char>) -> bool {
            if j == p.len() {
                return i == s.len();
            }

            let first = i < s.len() && (p[j] == s[i] || p[j] == '.');

            if j + 1 < p.len() && p[j + 1] == '*' {
                dfs(i, j + 2, s, p) ||
                (first && dfs(i + 1, j, s, p))
            } else {
                first && dfs(i + 1, j + 1, s, p)
            }
        }

        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        dfs(0, 0, &s, & p)
    }
}
