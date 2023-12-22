impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut ones = s.chars().filter(|&c| c == '1').count();
        let mut zeros = s.len() - ones;

        let mut left_zeros = 0;
        let mut right_ones = ones;
        let mut res = 0;
        for ch in s.chars() {
            if ch == '0' {
                left_zeros += 1;
            } else {
                right_ones -= 1;
            }
            if left_zeros == zeros && right_ones == 0 {
                break;
            }
            res = res.max(left_zeros + right_ones);
        }
        res as _
    }
}