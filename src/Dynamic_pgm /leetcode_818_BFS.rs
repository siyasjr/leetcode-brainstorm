use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let target = target as i32;
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((0, 1, 0)); // (position, speed, steps)
        visited.insert((0, 1));

        while let Some((pos, speed, steps)) = queue.pop_front() {
            if pos == target {
                return steps;
            }

            // Accelerate
            let next_pos = pos + speed;
            let next_speed = speed * 2;

            if next_pos > 0 && next_pos < target * 2 {
                if visited.insert((next_pos, next_speed)) {
                    queue.push_back((next_pos, next_speed, steps + 1));
                }
            }

            // Reverse
            let rev_speed = if speed > 0 { -1 } else { 1 };
            if visited.insert((pos, rev_speed)) {
                queue.push_back((pos, rev_speed, steps + 1));
            }
        }

        -1
    }
}

//complexity :O(target log target)