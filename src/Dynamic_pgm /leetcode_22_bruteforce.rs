impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        let len = (2 * n) as usize;

        fn valid(s: &str) -> bool {
            let mut bal = 0;
            for ch in s.chars() {
                if ch == '(' {
                    bal += 1;
                } else {
                    bal -= 1;
                }
                if bal < 0 { return false; }
            }
            bal == 0
        }

        fn gene(res: &mut Vec<String>, curr: &mut String, len: usize) {
            if curr.len() == len {
                if valid(curr) {
                    res.push(curr.clone());
                }
                return;
            }

            curr.push('(');
            gene(res, curr, len);
            curr.pop();

            curr.push(')');
            gene(res, curr, len);
            curr.pop();
        }

        let mut curr = String::new();
        gene(&mut res, &mut curr, len);
        res
    }
}
