use std::collections::HashMap;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = HashMap::new();
    for &num in &nums {
        *map.entry(num).or_insert(0) += 1;
        if map[&num] > 1 {
            return true;
        }
    }
    false
}

// Complexity: O(n)