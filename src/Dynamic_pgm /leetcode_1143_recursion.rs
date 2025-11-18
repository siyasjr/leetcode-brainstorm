use std::collections::HashMap;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        fn helper(
            i: usize,
            j: usize,
            a: &Vec<char>,
            b: &Vec<char>,
            memo: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if i == a.len() || j == b.len() { return 0; }
            if let Some(&v) = memo.get(&(i, j)) { return v; }

            let res = if a[i] == b[j] {
                1 + helper(i + 1, j + 1, a, b, memo)
            } else {
                helper(i + 1, j, a, b, memo).max(
                    helper(i, j + 1, a, b, memo)
                )
            };

            memo.insert((i, j), res);
            res
        }

        let a: Vec<char> = text1.chars().collect();
        let b: Vec<char> = text2.chars().collect();
        helper(0, 0, &a, &b, &mut HashMap::new())
    }
}

//cpmplexity: O(m·n)