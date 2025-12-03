impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        let n = s.len();
        let m = p.len();

        let mut prev = vec![false; m + 1];
        let mut curr = vec![false; m + 1];

        prev[0] = true;

        for j in 1..=m {
            if p[j - 1] == '*' && j >= 2 {
                prev[j] = prev[j - 2];
            }
        }

        for i in 1..=n {
            curr[0] = false;

            for j in 1..=m {
                if p[j - 1] == '*' {
                    curr[j] =
                        curr[j - 2]
                        || ((p[j - 2] == '.' || p[j - 2] == s[i - 1]) && prev[j]);
                } else {
                    curr[j] =
                        (p[j - 1] == '.' || p[j - 1] == s[i - 1])
                        && prev[j - 1];
                }
            }

            std::mem::swap(&mut prev, &mut curr);
        }

        prev[m]
    }
}

//complexity : O(n·m)
