fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    for i in 0..prices.len() {
        for j in i+1..prices.len() {
            let profit = prices[j] - prices[i];
            if profit > max_profit {
                max_profit = profit;
            }
        }
    }
    max_profit
}
