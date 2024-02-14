#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut pre = None;
        while let Some(mut val) = head.take(){
            head = val.next.take();
            val.next = pre.take();
            pre = Some(val);
        }
        pre
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    fn convert_list_into_vec(head: Option<Box<ListNode>>) -> Vec<i32>{
        let mut head = head;
        let mut res = Vec::new();
        while let Some(val) = head.take(){
            res.push(val.val);
            head = val.next;
        }
        res
    }
    #[test]
    fn test1(){
        let input = Some(Box::new(ListNode{val: 1, next:
                    Some(Box::new(ListNode{val: 2, next:
                    Some(Box::new(ListNode{val: 3, next:
                    Some(Box::new(ListNode{val: 4, next:
                    Some(Box::new(ListNode{val: 5, next: None}))}))}))}))}));
        let output = Solution::reverse_list(input);
        let output = convert_list_into_vec(output);
        assert_eq!(vec![5,4,3,2,1], output);
    }
}