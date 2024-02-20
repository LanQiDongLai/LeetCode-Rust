struct Solution;
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num < 10{
            return num;
        }
        let mut res = 0;
        let mut num = num;
        while num != 0{
            res += num % 10;
            num /= 10;
        }
        Self::add_digits(res)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input = 38;
        let output = Solution::add_digits(input);
        assert_eq!(2, output);
    }
    #[test]
    fn test2(){
        let input = 0;
        let output = Solution::add_digits(input);
        assert_eq!(0, output);
    }
}