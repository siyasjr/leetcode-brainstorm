impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        let mut jumps = 0;
        let mut curr_end = 0;
        let mut farthest = 0;

        for i in 0..n - 1 {
            farthest = farthest.max(i + nums[i] as usize);

            if i == curr_end {
                jumps += 1;
                curr_end = farthest;
            }
        }

        jumps
    }
}

//complexity : (O(n)