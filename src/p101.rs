struct Solution;

use super::structure::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root == None{
            return true;
        }
        let left = root.clone().unwrap().borrow().left.clone();
        let right = root.clone().unwrap().borrow().right.clone();
        Self::is_symmetric_tree(left, right)
    }
    fn is_symmetric_tree(q: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> bool {
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
        let b_left = Self::is_symmetric_tree(p_left, q_right);
        let b_right = Self::is_symmetric_tree(p_right, q_left);
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
        let input = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let output = Solution::is_symmetric(input);
        assert_eq!(true, output);
    }
    #[test]
    fn test2(){
        let input = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let output = Solution::is_symmetric(input);
        assert_eq!(false, output);
    }
}