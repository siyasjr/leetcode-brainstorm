use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.end = true;
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut root = Trie::default();
        for w in &word_dict {
            root.insert(w);
        }

        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 0..n {
            if !dp[i] { continue; }
            let mut node = &root;
            for j in i..n {
                if let Some(next) = node.children.get(&chars[j]) {
                    node = next;
                    if node.end {
                        dp[j + 1] = true;
                    }
                } else {
                    break;
                }
            }
        }
        dp[n]
    }
}
