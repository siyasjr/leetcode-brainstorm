impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn helper(n: i32) -> i32 {
            if n <= 1 { return 1; }
            helper(n - 1) + helper(n - 2)
        }
        helper(n)
    }
}
