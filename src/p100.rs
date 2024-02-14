// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}
struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none(){
            return true;
        }
        if p.is_none() || q.is_none(){
            return false;
        }
        let p_left = p.clone().unwrap().borrow().left.clone();
        let p_right = p.clone().unwrap().borrow().right.clone();
        let q_left = q.clone().unwrap().borrow().left.clone();
        let q_right = q.clone().unwrap().borrow().right.clone();
        let b_left = Self::is_same_tree(p_left, q_left);
        let b_right = Self::is_same_tree(p_right, q_right);
        let b_root = p.clone().unwrap().borrow().val == q.clone().unwrap().borrow().val;
        return if b_left && b_right && b_root {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input1 = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let input2 = input1.clone();
        let output = Solution::is_same_tree(input1, input2);
        assert_eq!(true, output)
    }
    #[test]
    fn test2(){
        let input1 = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        })));
        let input2 = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        let output = Solution::is_same_tree(input1, input2);
        assert_eq!(false, output)
    }
    #[test]
    fn test3(){
        let input1 = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: None,
                right: None,
            }))),
        })));
        let input2 = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        let output = Solution::is_same_tree(input1, input2);
        assert_eq!(false, output)
    }
}