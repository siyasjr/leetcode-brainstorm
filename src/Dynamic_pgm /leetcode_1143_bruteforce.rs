impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        fn helper(i: usize, j: usize, a: &Vec<char>, b: &Vec<char>) -> i32 {
            if i == a.len() || j == b.len() { return 0; }

            if a[i] == b[j] {
                1 + helper(i + 1, j + 1, a, b)
            } else {
                helper(i + 1, j, a, b).max(helper(i, j + 1, a, b))
            }
        }

        let a: Vec<char> = text1.chars().collect();
        let b: Vec<char> = text2.chars().collect();
        helper(0, 0, &a, &b)
    }
}

//complexity : O(2^(m+n))