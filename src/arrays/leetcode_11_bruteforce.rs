impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut max_area = 0;

        for i in 0..n {
            for j in i + 1..n {
                let area = (j - i) as i32 * height[i].min(height[j]);
                max_area = max_area.max(area);
            }
        }
        max_area
    }
}

// complexity : O(N²)