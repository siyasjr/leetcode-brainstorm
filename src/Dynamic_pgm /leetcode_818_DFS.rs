impl Solution {
    pub fn racecar(target: i32) -> i32 {
        fn dfs(pos: i32, speed: i32, target: i32, memo: &mut std::collections::HashMap<(i32,i32), i32>) -> i32 {
            if let Some(&v) = memo.get(&(pos, speed)) { return v; }
            if pos == target { return 0; }
            if pos < 0 || pos > 2 * target { return i32::MAX / 2; }

            // Accelerate
            let a = dfs(pos + speed, speed * 2, target, memo);

            // Reverse
            let r = dfs(pos, if speed > 0 { -1 } else { 1 }, target, memo);

            let best = (a + 1).min(r + 1);
            memo.insert((pos, speed), best);
            best
        }

        let mut memo = std::collections::HashMap::new();
        dfs(0, 1, target, &mut memo)
    }
}

//complexity :  exponential