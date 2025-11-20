impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut memo = vec![-1; (target + 1) as usize];

        fn dfs(nums: &Vec<i32>, t: i32, memo: &mut Vec<i32>) -> i32 {
            if t == 0 { return 1; }
            if t < 0 { return 0; }

            let idx = t as usize;
            if memo[idx] != -1 {
                return memo[idx];
            }

            let mut ways = 0;
            for &n in nums {
                ways += dfs(nums, t - n, memo);
            }

            memo[idx] = ways;
            ways
        }

        dfs(&nums, target, &mut memo)
    }
}

//complexity : O(n * target)**