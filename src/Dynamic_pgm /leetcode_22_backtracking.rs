impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(res: &mut Vec<String>, curr: String, open: i32, close: i32, n: i32) {
            if curr.len() == (n * 2) as usize {
                res.push(curr);
                return;
            }

            if open < n {
                backtrack(res, format!("{}(", curr), open + 1, close, n);
            }
            if close < open {
                backtrack(res, format!("{})", curr), open, close + 1, n);
            }
        }

        let mut res = Vec::new();
        backtrack(&mut res, String::new(), 0, 0, n);
        res
    }
}
