struct Solution;
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 2;
        if nums.len() <= 1{
            return 0;
        }
        if nums[nums.len() - 1] > nums[r as usize]{
            return nums.len() as i32 - 1;
        }
        while l < r{
            let mid = (l + r) / 2;
            let is_left = nums[mid as usize + 1] > nums[mid as usize];
            if is_left{
                l = mid + 1;
            }
            else{
                r = mid;
            }
        }
        l as i32
    }
}
#[cfg(test)]
mod tests{
    use std::collections::HashSet;
    use super::*;
    #[test]
    fn test1(){
        let input = vec![1,2,3,1];
        let output = Solution::find_peak_element(input);
        let mut sample = HashSet::new();
        sample.insert(2);
        assert!(sample.contains(&output));
    }
    #[test]
    fn test2(){
        let input = vec![1,2,1,3,5,6,4];
        let output = Solution::find_peak_element(input);
        let mut sample = HashSet::new();
        sample.insert(1);
        sample.insert(5);
        assert!(sample.contains(&output));
    }
}