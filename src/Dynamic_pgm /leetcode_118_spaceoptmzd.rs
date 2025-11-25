impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut res = Vec::with_capacity(n);
        let mut prev = vec![];

        for i in 0..n {
            let mut row = vec![1; i + 1];

            for j in 1..i {
                row[j] = prev[j - 1] + prev[j];
            }

            prev = row.clone();
            res.push(row);
        }

        res
    }
}

//complexity : O(n²)