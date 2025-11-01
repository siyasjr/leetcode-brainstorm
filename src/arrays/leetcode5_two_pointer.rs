pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let n = nums.len();
    let mut res = Vec::new();

    for i in 0..n {
        if i > 0 && nums[i] == nums[i - 1] {
            continue; // skip duplicates for the first element
        }

        let (mut l, mut r) = (i + 1, n - 1);

        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            if sum < 0 {
                l += 1;
            } else if sum > 0 {
                r -= 1;
            } else {
                res.push(vec![nums[i], nums[l], nums[r]]);
                l += 1;
                r -= 1;

                while l < r && nums[l] == nums[l - 1] {
                    l += 1; // skip duplicates
                }
                while l < r && nums[r] == nums[r + 1] {
                    r -= 1; // skip duplicates
                }
            }
        }
    }

    res
}
