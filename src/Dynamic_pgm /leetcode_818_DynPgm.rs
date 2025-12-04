impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];

        for t in 1..=target {
            dp[t] = i32::MAX;

            let mut k = 1;
            let mut dist_k = (1 << k) - 1; // distance after k accelerations

            // Case 1 & 2: overshoot or exact
            while dist_k < 2 * t {
                if dist_k == t {
                    dp[t] = k as i32;
                } else if dist_k > t {
                    // overshoot: K A’s, R, then dp[dist_k - t]
                    dp[t] = dp[t].min((k as i32) + 1 + dp[dist_k - t]);
                } else {
                    // Case 3: undershoot → reverse before overshooting
                    // Try all possible j accelerations after reversing
                    for j in 0..k {
                        let dist_j = (1 << j) - 1;
                        dp[t] = dp[t].min((k as i32) + 1 + (j as i32) + 1 + dp[t - (dist_k - dist_j)]);
                    }
                }

                k += 1;
                dist_k = (1 << k) - 1;
            }
        }

        dp[target]
    }
}
