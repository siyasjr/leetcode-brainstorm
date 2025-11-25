impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        fn calc(i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if j == 0 || j == i {
                return 1;
            }
            if memo[i][j] != 0 {
                return memo[i][j];
            }
            memo[i][j] = calc(i - 1, j - 1, memo) + calc(i - 1, j, memo);
            memo[i][j]
        }

        let n = num_rows as usize;
        let mut memo = vec![vec![0; n + 1]; n + 1];
        let mut res = vec![];

        for i in 0..n {
            let mut row = vec![];
            for j in 0..=i {
                row.push(calc(i, j, &mut memo));
            }
            res.push(row);
        }

        res
    }
}
