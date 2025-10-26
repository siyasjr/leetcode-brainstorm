impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];

        // Step 1: prefix products
        for i in 1..n {
            res[i] = res[i - 1] * nums[i - 1];
        }

        // Step 2: suffix products
        let mut suffix = 1;
        for i in (0..n).rev() {
            res[i] *= suffix;
            suffix *= nums[i];
        }

        res
    }
}
