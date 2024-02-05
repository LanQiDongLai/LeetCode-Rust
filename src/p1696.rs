struct Solution;
impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut heap = std::collections::BinaryHeap::new();
        for (i, x) in nums.into_iter().enumerate() {
            let i = i as i32;
            let mut cur = 0;
            while let Some(&(t, idx)) = heap.peek() {
                if i - idx > k  {
                    heap.pop();
                }
                else {
                    cur = t;
                    break
                }
            }
            heap.push((cur + x, i));
        }
        while let Some((t, idx)) = heap.pop() {
            if idx == n as i32 - 1 {return t}
        }
        0
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input1 = vec![1,-1,-2,4,-7,3];
        let input2 = 2;
        let output = Solution::max_result(input1, input2);
        assert_eq!(output, 7);
    }
    #[test]
    fn test2(){
        let input1 = vec![10,-5,-2,4,0,3];
        let input2 = 3;
        let output = Solution::max_result(input1, input2);
        assert_eq!(output, 17);
    }
    #[test]
    fn test3(){
        let input1 = vec![1,-5,-20,4,-1,3,-6,-3];
        let input2 = 2;
        let output = Solution::max_result(input1, input2);
        assert_eq!(output, 0);
    }
}