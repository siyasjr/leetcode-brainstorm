fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_current = 0;
    let mut max_global = 0;

    for i in 1..prices.len() {
        let diff = prices[i] - prices[i - 1];
        max_current = (max_current + diff).max(0);
        max_global = max_global.max(max_current);
    }

    max_global
}
