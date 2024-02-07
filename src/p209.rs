struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut tsum = 0;
        let mut res = 0x3f3f3f3f;
        while r <= nums.len(){
            if tsum < target{
                if r == nums.len(){
                    break
                }
                r += 1;
                tsum += nums[r-1];
            }
            else{
                res = res.min((r - l) as i32);
                l += 1;
                tsum -= nums[l - 1];
            }
        }
        if res == 0x3f3f3f3f{
            0
        }
        else{
            res
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input1 = 7;
        let input2 = vec![2,3,1,2,4,3];
        let output = Solution::min_sub_array_len(input1, input2);
        assert_eq!(2, output);
    }
    #[test]
    fn test2(){
        let input1 = 4;
        let input2 = vec![1,4,4];
        let output = Solution::min_sub_array_len(input1, input2);
        assert_eq!(1, output);
    }
    #[test]
    fn test3(){
        let input1 = 11;
        let input2 = vec![1,1,1,1,1,1,1,1];
        let output = Solution::min_sub_array_len(input1, input2);
        assert_eq!(0, output);
    }
}