impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn solve(n: i32, memo: &mut Vec<i32>) -> i32 {
            if n <= 1 { return 1; }
            if memo[n as usize] != -1 {
                return memo[n as usize];
            }
            memo[n as usize] = solve(n - 1, memo) + solve(n - 2, memo);
            memo[n as usize]
        }

        let mut memo = vec![-1; (n + 1) as usize];
        solve(n, &mut memo)
    }
}
