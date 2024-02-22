struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut line = vec![1];
        for _ in 0..num_rows{
            res.push(line.clone());
            line = line.windows(2).map(|arr|arr[0] + arr[1]).collect();
            line.insert(0, 1);
            line.push(1);
        }
        res
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input = 5;
        let output = Solution::generate(input);
        assert_eq!(vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]], output);
    }
    #[test]
    fn test2(){
        let input = 1;
        let output = Solution::generate(input);
        assert_eq!(vec![vec![1]], output);
    }
}