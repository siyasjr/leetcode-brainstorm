impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut current_sum = nums[0];

        for &num in nums.iter().skip(1) {
            current_sum = current_sum.max(current_sum + num).max(num);
            max_sum = max_sum.max(current_sum);
        }

        max_sum
    }
}

// Complexity - (Optimal — O(n), O(1)) 

/*
Steps:

Initialize max_sum = nums[0]

Initialize current_sum = nums[0]

For each num in nums[1..]:

current_sum = max(num, current_sum + num)

max_sum = max(max_sum, current_sum)

Return max_sum
 */