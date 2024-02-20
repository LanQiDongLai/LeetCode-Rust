struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut nxt = String::new();
        for ch in s.chars(){
            if ch.is_alphabetic() || ch.is_digit(10){
                let mut ch = ch.to_lowercase();
                let ch = ch.next().unwrap();
                nxt.push(ch);
            }
        }
        let rev = nxt.chars().rev().collect::<String>();
        rev == nxt
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input = "A man, a plan, a canal: Panama".to_string();
        let output = Solution::is_palindrome(input);
        assert_eq!(true, output);
    }
    #[test]
    fn test2(){
        let input = "race a car".to_string();
        let output = Solution::is_palindrome(input);
        assert_eq!(false, output);
    }
    #[test]
    fn test3(){
        let input = " ".to_string();
        let output = Solution::is_palindrome(input);
        assert_eq!(true, output);
    }
}