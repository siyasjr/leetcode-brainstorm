impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut dp: Vec<Vec<String>> = vec![vec![]; n + 1];
        dp[0] = vec![String::from("")];

        for i in 1..=n {
            let mut curr = Vec::new();
            for left in 0..i {
                for a in dp[left].iter() {
                    for b in dp[i - 1 - left].iter() {
                        curr.push(format!("({}){}", a, b));
                    }
                }
            }
            dp[i] = curr;
        }

        dp[n].clone()
    }
}

//complexity : O(4ⁿ / √n)
