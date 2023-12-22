impl Solution {
    pub fn two_sum(price: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = price.len() - 1;
        while left < right {
            let sum = price[left] + price[right];
            if sum == target {
                return vec![price[left], price[right]];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        vec![]
    }
}