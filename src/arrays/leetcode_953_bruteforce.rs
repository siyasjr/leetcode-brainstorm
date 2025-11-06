use std::collections::HashMap;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut rank = HashMap::new();
        for (i, c) in order.chars().enumerate() {
            rank.insert(c, i);
        }

        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if !Self::in_order(&words[i], &words[j], &rank) {
                    return false;
                }
            }
        }
        true
    }

    fn in_order(a: &str, b: &str, rank: &HashMap<char, usize>) -> bool {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();

        for (ca, cb) in a_chars.iter().zip(b_chars.iter()) {
            if ca != cb {
                return rank.get(ca).unwrap() < rank.get(cb).unwrap();
            }
        }
        a.len() <= b.len()
    }
}


// complexity : O(N² × L)