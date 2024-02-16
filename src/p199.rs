struct Solution;
use crate::structure::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        Self::right_side_view_tree(root, 0, &mut res);
        res
    }
    pub fn right_side_view_tree(root: Option<Rc<RefCell<TreeNode>>>, depth: u32, res: &mut Vec<i32>){
        if root == None{
            return;
        }
        if depth == res.len() as u32{
            res.push(root.clone().unwrap().borrow().val);
        }
        let left = root.clone().unwrap().borrow().left.clone();
        let right = root.clone().unwrap().borrow().right.clone();
        Self::right_side_view_tree(right, depth + 1, res);
        Self::right_side_view_tree(left, depth + 1, res);
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
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let output = Solution::right_side_view(input);
        assert_eq!(vec![1,3,4], output);
    }
    #[test]
    fn test2(){
        let input = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let output = Solution::right_side_view(input);
        assert_eq!(vec![1,3], output);
    }
    #[test]
    fn test3(){
        let input = None;
        let output = Solution::right_side_view(input);
        assert_eq!(Vec::<i32>::new(), output);
    }
}