impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut slow = 0;
        let mut fast = 0;
        while fast < nums.len() {
            if nums[slow] != nums[fast] {
                slow += 1;
                nums[slow] = nums[fast];
            }
            fast += 1;
        }
        slow as i32 + 1
    }
}