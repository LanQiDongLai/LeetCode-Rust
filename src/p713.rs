struct Solution;
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut tmul = 1;
        let mut res = 0;
        for l in 0..nums.len(){
            let mut r = l;
            while r < nums.len(){
                tmul *= nums[r];
                r += 1;
                if tmul >= k{
                    break;
                }
                res += 1;
            }
            tmul = 1;
        }
        res
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input1 = vec![10,5,2,6];
        let input2 = 100;
        let output = Solution::num_subarray_product_less_than_k(input1, input2);
        assert_eq!(8, output);
    }
    #[test]
    fn test2(){
        let input1 = vec![1,2,3];
        let input2 = 0;
        let output = Solution::num_subarray_product_less_than_k(input1, input2);
        assert_eq!(0, output);
    }
}