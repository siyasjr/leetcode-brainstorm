impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        fn dfs(i: usize, j: usize, m: usize, n: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if i == m - 1 && j == n - 1 { return 1; }
            if memo[i][j] != -1 { return memo[i][j]; }

            let mut ans = 0;
            if i + 1 < m {
                ans += dfs(i + 1, j, m, n, memo);
            }
            if j + 1 < n {
                ans += dfs(i, j + 1, m, n, memo);
            }
            memo[i][j] = ans;
            ans
        }

        let m = m as usize;
        let n = n as usize;
        let mut memo = vec![vec![-1; n]; m];

        dfs(0, 0, m, n, &mut memo)
    }
}

//complexity : O(m·n)