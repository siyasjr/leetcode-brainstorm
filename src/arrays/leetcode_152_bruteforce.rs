impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_prod = i32::MIN;

        for i in 0..n {
            let mut prod = 1;
            for j in i..n {
                prod *= nums[j];
                max_prod = max_prod.max(prod);
            }
        }

        max_prod
    }
}

// COmplexity : Time: O(n²)