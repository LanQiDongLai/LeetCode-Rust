use crate::structure::TreeNode;

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root == None{
            return true;
        }
        Self::depth_tree(root) != -1
    }
    pub fn depth_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32{
        if root == None{
            return 0;
        }
        let left = root.clone().unwrap().borrow().left.clone();
        let right = root.clone().unwrap().borrow().right.clone();
        let left_depth = Self::depth_tree(left) as i32;
        let right_depth = Self::depth_tree(right) as i32;
        if left_depth == -1 || right_depth == -1{
            return -1;
        }
        if (left_depth - right_depth).abs() > 1{
            return -1;
        }
        left_depth.max(right_depth) + 1
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input = Some(Rc::new(RefCell::new(TreeNode{
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let output = Solution::is_balanced(input);
        assert_eq!(true, output);
    }
    #[test]
    fn test2(){
        let input = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 4,
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
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        let output = Solution::is_balanced(input);
        assert_eq!(false, output);
    }
    #[test]
    fn test3(){
        let input = None;
        let output = Solution::is_balanced(input);
        assert_eq!(true, output);
    }
    #[test]
    fn test4(){
        let input = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: None,
            right: None,
        })));
        let output = Solution::is_balanced(input);
        assert_eq!(true, output);
    }
}