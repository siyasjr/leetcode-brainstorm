impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_prod = i32::MIN;
        let mut prod = 1;

        // left to right
        for &num in &nums {
            prod *= num;
            max_prod = max_prod.max(prod);
            if num == 0 { prod = 1; }
        }

        // right to left
        prod = 1;
        for &num in nums.iter().rev() {
            prod *= num;
            max_prod = max_prod.max(prod);
            if num == 0 { prod = 1; }
        }Multiply left-to-right and right-to-left — resets when zero appears.

        max_prod
    }
}
