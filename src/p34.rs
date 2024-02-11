struct Solution;
impl Solution{
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = nums.len() as i32;
        let mut res = Vec::new();
        while l < r{
            let mid = (l + r) / 2;
            if nums[mid as usize] < target{
                l = mid + 1;
            }
            else{
                r = mid;
            }
        }
        res.push(r);
        l = 0;
        r = nums.len() as i32;
        while l < r{
            let mid = (l + r) / 2;
            if nums[mid as usize] <= target{
                l = mid + 1;
            }
            else{
                r = mid;
            }
        }
        res.push(r - 1);
        if res[0] > res[1]{
            res = vec![-1, -1];
        }
        res
    }
}
/*
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.binary_search(&target).is_err() {
            vec![-1, -1]
        } else {
            vec![
                nums.partition_point(|&x| x < target) as i32,
                nums.partition_point(|&x| x <= target) as i32 - 1,
            ]
        }
    }
}
 */
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input1 = vec![5,7,7,8,8,10];
        let input2 = 8;
        let output = Solution::search_range(input1, input2);
        assert_eq!(vec![3,4], output);
    }
    #[test]
    fn test2(){
        let input1 = vec![5,7,7,8,8,10];
        let input2 = 6;
        let output = Solution::search_range(input1, input2);
        assert_eq!(vec![-1,-1], output);
    }
    #[test]
    fn test3(){
        let input1 = vec![];
        let input2 = 0;
        let output = Solution::search_range(input1, input2);
        assert_eq!(vec![-1,-1], output);
    }
}