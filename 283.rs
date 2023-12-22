impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut slow = 0;
        let mut fast = 0;
        while fast < nums.len() {
            if nums[fast] != 0 {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }

        for i in slow..nums.len() {
            nums[i] = 0;
        }
    }
}