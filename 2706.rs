impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut prices = prices;
        prices.sort();
        let amount = prices[0] + prices[1];
        if amount <= money {
            money - amount
        } else {
            money
        }
    }
}