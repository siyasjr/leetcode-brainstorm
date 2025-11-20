impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        fn dfs(nums: &Vec<i32>, target: i32) -> i32 {
            if target == 0 { return 1; }
            if target < 0 { return 0; }

            let mut count = 0;
            for &n in nums {
                count += dfs(nums, target - n);
            }
            count
        }

        dfs(&nums, target)
    }
}
