impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (s1, s2) = if text1.len() < text2.len() {
            (text1, text2)
        } else {
            (text2, text1) // s1 is always shorter
        };

        let a: Vec<char> = s1.chars().collect();
        let b: Vec<char> = s2.chars().collect();
        let m = a.len();
        let n = b.len();

        let mut prev = vec![0; m + 1];
        let mut curr = vec![0; m + 1];

        for j in (0..n).rev() {
            for i in (0..m).rev() {
                curr[i] = if b[j] == a[i] {
                    1 + prev[i + 1]
                } else {
                    prev[i].max(curr[i + 1])
                }
            }
            std::mem::swap(&mut prev, &mut curr);
        }

        prev[0]
    }
}
