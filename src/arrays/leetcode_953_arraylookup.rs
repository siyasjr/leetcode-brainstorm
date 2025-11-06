impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut rank = [0usize; 26];
        for (i, c) in order.chars().enumerate() {
            rank[(c as u8 - b'a') as usize] = i;
        }

        for i in 0..words.len() - 1 {
            if !Self::compare(&words[i], &words[i + 1], &rank) {
                return false;
            }
        }
        true
    }

    fn compare(a: &str, b: &str, rank: &[usize; 26]) -> bool {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();

        for (&ca, &cb) in a_bytes.iter().zip(b_bytes.iter()) {
            if ca != cb {
                return rank[(ca - b'a') as usize] < rank[(cb - b'a') as usize];
            }
        }
        a.len() <= b.len()
    }
}

// complexity : O(N * L)