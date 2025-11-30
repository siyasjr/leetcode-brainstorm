use std::collections::VecDeque;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        let mut visited = vec![false; n];
        visited[0] = true;

        let mut queue = VecDeque::new();
        queue.push_back((0, 0)); // (index, steps)

        while let Some((i, steps)) = queue.pop_front() {
            if i == n - 1 {
                return steps;
            }

            let max_jump = nums[i] as usize;

            for j in i+1..=i+max_jump {
                if j < n && !visited[j] {
                    visited[j] = true;
                    queue.push_back((j, steps + 1));
                }
            }
        }

        0
    }
}

//complexity : O(n²)