struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        let mut res = 0;
        let mut count = 0;
        while x != 0{
            res *= 2;
            if x % 2 == 1{
                res += 1;
            }
            x /= 2;
            count += 1;
        }
        for _ in 0..(32-count){
            res *= 2;
        }
        res
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input = 0b00000010100101000001111010011100;
        let output = Solution::reverse_bits(input);
        assert_eq!(964176192, output);
    }
    #[test]
    fn test2(){
        let input = 0b11111111111111111111111111111101;
        let output = Solution::reverse_bits(input);
        assert_eq!(3221225471, output);
    }
}