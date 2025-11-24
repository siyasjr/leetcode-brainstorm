impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        fn dfs(i: i32, j: i32, m: i32, n: i32) -> i32 {
            if i == m - 1 && j == n - 1 { return 1; }
            if i >= m || j >= n { return 0; }
            dfs(i + 1, j, m, n) + dfs(i, j + 1, m, n)
        }

        dfs(0, 0, m, n)
    }
}

//complexity : O(2^(m+n))