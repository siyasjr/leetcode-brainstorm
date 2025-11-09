impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2)
    }
}

// complexity : O(2ⁿ)