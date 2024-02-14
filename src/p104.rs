struct Solution;

use super::structure::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root;
        if root == None{
            return 0;
        }
        let left = root.clone().unwrap().borrow().left.clone();
        let right = root.clone().unwrap().borrow().right.clone();
        (Self::max_depth(left) + 1).max(Self::max_depth(right) + 1)
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
        let output = Solution::max_depth(input);
        assert_eq!(3, output);
    }
    #[test]
    fn test2(){
        let input = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: None,
                right: None,
            })))
        })));
        let output = Solution::max_depth(input);
        assert_eq!(2, output);
    }
}