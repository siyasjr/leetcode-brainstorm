impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_prod = nums[0];
        let mut cur_max = nums[0];
        let mut cur_min = nums[0];

        for &num in nums.iter().skip(1) {
            let temp = cur_max;
            cur_max = num.max(num * cur_max).max(num * cur_min);
            cur_min = num.min(num * temp).min(num * cur_min);
            max_prod = max_prod.max(cur_max);
        }

        max_prod
    }
}
