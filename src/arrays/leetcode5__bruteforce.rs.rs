use std::collections::HashSet;

pub fn three_sum_bruteforce(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums.clone();
    let n = nums.len();
    let mut result = HashSet::new();

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if nums[i] + nums[j] + nums[k] == 0 {
                    let mut triplet = vec![nums[i], nums[j], nums[k]];
                    triplet.sort();
                    result.insert(triplet);
                }
            }
        }
    }

    result.into_iter().collect()
}
