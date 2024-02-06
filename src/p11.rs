struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut res = 0;
        while l < r{
            res = res.max(height[l].min(height[r]) * (r - l) as i32);
            if height[l] < height[r]{
                l += 1;
            }
            else{
                r -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input = vec![1,8,6,2,5,4,8,3,7];
        let output = Solution::max_area(input);
        assert_eq!(49, output);
    }
    #[test]
    fn test2(){
        let input = vec![1,1];
        let output = Solution::max_area(input);
        assert_eq!(1, output);
    }
}