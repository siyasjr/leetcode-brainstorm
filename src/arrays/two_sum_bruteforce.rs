fn two_sum_bruteforce(nums: Vec<i32>, target: i32) -> Vec<usize> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i, j];
            }
        }
    }
    vec![]
}

// Time Complexity : O(n²)