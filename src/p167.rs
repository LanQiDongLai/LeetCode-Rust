struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;
        while l < r {
            if numbers[l] + numbers[r] > target{
                r -= 1;
            }
            else if numbers[l] + numbers[r] < target {
                l += 1;
            }
            else{
                break
            }
        }
        vec![l as i32 + 1, r as i32 + 1]
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input1 = vec![2,7,11,15];
        let input2 = 9;
        let output = Solution::two_sum(input1, input2);
        assert_eq!(vec![1,2], output);
    }
    #[test]
    fn test2(){
        let input1 = vec![2,3,4];
        let input2 = 6;
        let output = Solution::two_sum(input1, input2);
        assert_eq!(vec![1,3], output);
    }
    #[test]
    fn test3(){
        let input1 = vec![-1, 0];
        let input2 = -1;
        let output = Solution::two_sum(input1, input2);
        assert_eq!(vec![1,2], output);
    }
}