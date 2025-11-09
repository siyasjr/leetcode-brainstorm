use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn helper(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if n <= 2 {
                return n;
            }
            if let Some(&val) = memo.get(&n) {
                return val;
            }
            let val = helper(n - 1, memo) + helper(n - 2, memo);
            memo.insert(n, val);
            val
        }

        let mut memo = HashMap::new();
        helper(n, &mut memo)
    }
}

//complexity : O(n)
