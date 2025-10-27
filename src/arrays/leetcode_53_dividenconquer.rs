ub fn max_sub_array(nums: Vec<i32>) -> i32 {
    fn helper(nums: &[i32], left: usize, right: usize) -> i32 {
        if left == right {
            return nums[left];
        }

        let mid = (left + right) / 2;
        let left_max = helper(nums, left, mid);
        let right_max = helper(nums, mid + 1, right);
        let cross_max = cross_sum(nums, left, mid, right);

        left_max.max(right_max).max(cross_max)
        
    }
}