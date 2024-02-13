struct Solution;
impl Solution{
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let is_blue = |index| -> bool{
            let end = nums[nums.len() - 1];
            return if nums[index] > end {
                target > end && nums[index] >= target
            } else {
                target > end || nums[index] >= target
            }
        };
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r{
            let mid = (l + r) / 2;
            if is_blue(mid){
                r = mid;
            }
            else{
                l = mid + 1;
            }
        }
        if nums[r] != target{
            return -1;
        }
        r as i32
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input1 = vec![4,5,6,7,0,1,2];
        let input2 = 0;
        let output = Solution::search(input1, input2);
        assert_eq!(4, output);
    }
    #[test]
    fn test2(){
        let input1 = vec![4,5,6,7,0,1,2];
        let input2 = 3;
        let output = Solution::search(input1, input2);
        assert_eq!(-1, output);
    }
    #[test]
    fn test3(){
        let input1 = vec![1];
        let input2 = 0;
        let output = Solution::search(input1, input2);
        assert_eq!(-1, output);
    }
}