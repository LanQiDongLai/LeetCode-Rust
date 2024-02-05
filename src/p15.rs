struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut res = Vec::new();
        for i in 0..n-2{
            let x = nums[i];
            if i > 0 && x == nums[i - 1]{
                continue;
            }
            if x + nums[i] + nums[i + 1] > 0{
                break;
            }
            if x + nums[n - 1] + nums[n - 2] < 0{
                continue;
            }
            let mut j = i + 1;
            let mut k = n - 1;
            while j < k{
                if x + nums[j] + nums[k] < 0{
                    j += 1;
                }
                else if x + nums[j] + nums[k] > 0{
                    k -= 1;
                }
                else{
                    res.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    while j < k && nums[j] == nums[j-1]{
                        j += 1;
                    }
                    k -= 1;
                    while j < k && nums[k] == nums[k+1]{
                        k -= 1;
                    }
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test1(){
        let input = vec![-1,0,1,2,-1,-4];
        let mut output = Solution::three_sum(input);
        for arr in &mut output{
            arr.sort();
        }
        output.sort();
        let mut sample = vec![vec![-1,-1,2],vec![-1,0,1]];
        for arr in &mut sample{
            arr.sort();
        }
        sample.sort();
        assert_eq!(sample, output);
    }
    #[test]
    fn test2(){
        let input = vec![0,1,1];
        let mut output = Solution::three_sum(input);
        for arr in &mut output{
            arr.sort();
        }
        output.sort();
        let mut sample:Vec<Vec<i32>> = vec![];
        for arr in &mut sample{
            arr.sort();
        }
        sample.sort();
        assert_eq!(sample, output);
    }
    #[test]
    fn test3(){
        let input = vec![0,0,0];
        let mut output = Solution::three_sum(input);
        for arr in &mut output{
            arr.sort();
        }
        output.sort();
        let mut sample:Vec<Vec<i32>> = vec![vec![0,0,0]];
        for arr in &mut sample{
            arr.sort();
        }
        sample.sort();
        assert_eq!(sample, output);
    }
}