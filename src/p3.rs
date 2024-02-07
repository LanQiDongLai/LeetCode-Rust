struct Solution;
impl Solution{
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut tmap = std::collections::HashSet::<char>::new();
        let arr: Vec<_> = s.chars().collect();
        let mut res = 0;
        for l in 0..arr.len(){
            let mut r = l;
            while r < s.len(){
                if tmap.contains(&arr[r]) == true{
                    tmap.clear();
                    break;
                }
                else{
                    res = res.max((r - l + 1) as i32);
                    tmap.insert(arr[r]);
                    r += 1;
                }
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
        let input = String::from("abcabcbb");
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(3, output);
    }
    #[test]
    fn test2(){
        let input = String::from("bbbbb");
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(1, output);
    }
    #[test]
    fn test3(){
        let input = String::from("pwwkew");
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(3, output);
    }
}