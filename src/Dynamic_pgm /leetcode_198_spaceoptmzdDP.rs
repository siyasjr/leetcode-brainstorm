impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev1 = 0; // dp[i-1]
        let mut prev2 = 0; // dp[i-2]

        for n in nums {
            let rob = prev2 + n;
            let skip = prev1;
            let cur = rob.max(skip);

            prev2 = prev1;
            prev1 = cur;
        }

        prev1
    }
}

//complexity : O(n)**