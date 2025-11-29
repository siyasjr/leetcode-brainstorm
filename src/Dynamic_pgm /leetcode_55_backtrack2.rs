impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn dfs(res: &mut Vec<String>, curr: &mut String, open: i32, close: i32, n: i32) {
            if curr.len() == (2 * n) as usize {
                res.push(curr.clone());
                return;
            }

            if open < n {
                curr.push('(');
                dfs(res, curr, open + 1, close, n);
                curr.pop();
            }

            if close < open {
                curr.push(')');
                dfs(res, curr, open, close + 1, n);
                curr.pop();
            }
        }

        let mut res = vec![];
        let mut curr = String::new();
        dfs(&mut res, &mut curr, 0, 0, n);
        res
    }
}
