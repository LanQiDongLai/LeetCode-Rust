struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let mut input = vec![1,1,2];
        let output = Solution::remove_duplicates(&mut input);
        assert_eq!(2, output);
        let sample = vec![1, 2];
        for (index, &num) in sample.iter().enumerate(){
            assert_eq!(num, input[index])
        }
    }

    #[test]
    fn test2(){
        let mut input = vec![0,0,1,1,1,2,2,3,3,4];
        let output = Solution::remove_duplicates(&mut input);
        assert_eq!(5, output);
        let sample = vec![0, 1, 2, 3, 4];
        for (index, &num) in sample.iter().enumerate(){
            assert_eq!(num, input[index])
        }
    }
}