impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        fn backtrack(s: &str, dict: &Vec<String>) -> bool {
            if s.is_empty() {
                return true;
            }
            for w in dict {
                if s.starts_with(w) {
                    if backtrack(&s[w.len()..], dict) {
                        return true;
                    }
                }
            }
            false
        }

        backtrack(&s, &word_dict)
    }
}
