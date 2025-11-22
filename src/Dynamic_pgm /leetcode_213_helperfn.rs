impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 { return 0; }
        if n == 1 { return nums[0]; }

        // helper to rob a linear row of houses
        fn rob_line(nums: &[i32]) -> i32 {
            let mut prev1 = 0;
            let mut prev2 = 0;

            for &n in nums {
                let cur = (prev2 + n).max(prev1);
                prev2 = prev1;
                prev1 = cur;
            }
            prev1
        }

        // case 1: exclude last house
        let case1 = rob_line(&nums[..n - 1]);

        // case 2: exclude first house
        let case2 = rob_line(&nums[1..]);

        case1.max(case2)
    }
}
