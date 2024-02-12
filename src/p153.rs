struct Solution;
impl Solution{
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r{
            let mid = (l + r) / 2;
            if nums[mid] > nums[r] && nums[mid] > nums[l]{
                l = mid + 1;
            }
            else if nums[mid] < nums[r] && nums[mid] < nums[l]{
                r = mid;
            }
            else{
                return nums[l];
            }
        }
        nums[l]
    }
}
/*
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 || nums[0] < nums[n - 1] {
            nums[0]
        } else {
            nums[nums.partition_point(|&x| x >= nums[0])]
        }
    }
}
 */
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input = vec![3,4,5,1,2];
        let output = Solution::find_min(input);
        assert_eq!(1, output);
    }
    #[test]
    fn test2(){
        let input = vec![4,5,6,7,0,1,2];
        let output = Solution::find_min(input);
        assert_eq!(0, output);
    }
    #[test]
    fn test3(){
        let input = vec![11,13,15,17];
        let output = Solution::find_min(input);
        assert_eq!(11, output);
    }
}