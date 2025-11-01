use std::collections::HashSet;

pub fn three_sum_hash(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums.clone();
    let n = nums.len();
    let mut result = HashSet::new();

    for i in 0..n {
        let mut seen = HashSet::new();
        for j in i + 1..n {
            let complement = -nums[i] - nums[j];
            if seen.contains(&complement) {
                let mut triplet = vec![nums[i], nums[j], complement];
                triplet.sort();
                result.insert(triplet);
            }
            seen.insert(nums[j]);
        }
    }

    result.into_iter().collect()
}

//Complexity : O(n²)