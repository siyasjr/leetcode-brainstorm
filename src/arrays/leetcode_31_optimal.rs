impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n < 2 { return; }

        // 1️⃣ Find first decreasing element from the right
        let mut i = (n - 2) as isize;
        while i >= 0 && nums[i as usize] >= nums[i as usize + 1] {
            i -= 1;
        }

        if i >= 0 {
            // 2️⃣ Find the next larger element to swap
            let mut j = (n - 1) as isize;
            while nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            nums.swap(i as usize, j as usize);
        }

        // 3️⃣ Reverse the tail
        nums[(i + 1) as usize..].reverse();
    }
}
