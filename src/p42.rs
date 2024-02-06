struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut ml = 0;
        let mut mr = 0;
        let mut res = 0;
        while l < r{
            if ml < mr{
                res += (ml - height[l]).max(0);
                ml = ml.max(height[l]);
                l += 1;
            }
            else{
                res += (mr - height[r]).max(0);
                mr = mr.max(height[r]);
                r -= 1;
            }
        }
        res += (ml.min(mr) - height[l]).max(0);
        res
    }
}
/*
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut pre_max = Vec::new();
        let mut sub_max = Vec::new();
        let n = height.len();
        let mut maxn = 0;
        for num in height.iter(){
            maxn = maxn.max(*num);
            pre_max.push(maxn);
        }
        maxn = 0;
        for num in height.iter().rev(){
            maxn = maxn.max(*num);
            sub_max.push(maxn);
        }
        sub_max.reverse();
        let mut res = 0;
        for i in 0..n{
            res += pre_max[i].min(sub_max[i]) - height[i];
        }
        res
    }
}*/
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let input = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let output = Solution::trap(input);
        assert_eq!(6, output);
    }
    #[test]
    fn test2(){
        let input = vec![4,2,0,3,2,5];
        let output = Solution::trap(input);
        assert_eq!(9, output);
    }
    #[test]
    fn test3(){
        let input = vec![0];
        let output = Solution::trap(input);
        assert_eq!(0, output);
    }
}