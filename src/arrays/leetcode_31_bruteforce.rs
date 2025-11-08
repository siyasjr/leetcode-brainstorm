use itertools::Itertools;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut perms: Vec<_> = nums.clone().into_iter().permutations(nums.len()).collect();
        perms.sort();
        for i in 0..perms.len() {
            if perms[i] == *nums && i + 1 < perms.len() {
                *nums = perms[i + 1].clone();
                return;
            }
        }
        nums.sort();
    }
}
