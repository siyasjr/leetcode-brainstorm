fn two_sum_sorting(mut nums: Vec<(i32, usize)>, target: i32) -> Vec<usize> {
    // Sort by values
    nums.sort_by_key(|x| x.0);

    let (mut left, mut right) = (0, nums.len() - 1);

    while left < right {
        let sum = nums[left].0 + nums[right].0;
        if sum == target {
            return vec![nums[left].1, nums[right].1];
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let indexed: Vec<_> = nums.iter().cloned().enumerate().map(|(i, v)| (v, i)).collect();
    println!("{:?}", two_sum_sorting(indexed, target)); // [0, 1]
}
