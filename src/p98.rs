struct Solution;
use crate::structure::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut res = true;
        Self::is_valid_bst_tree(root, &mut res);
        res
    }
    pub fn is_valid_bst_tree(root: Option<Rc<RefCell<TreeNode>>>, is_valid_bst: &mut bool) -> Option<(i32, i32)>{
        if root == None{
            return None;
        }
        let left = root.clone().unwrap().borrow().left.clone();
        let right = root.clone().unwrap().borrow().right.clone();
        let val = root.clone().unwrap().borrow().val;
        let r_left = Self::is_valid_bst_tree(left, is_valid_bst);
        let r_right = Self::is_valid_bst_tree(right, is_valid_bst);
        let mut res = (0,0);
        *is_valid_bst = match (r_left, r_right){
            (Some(l), Some(r)) =>{
                if l.1 >= val || r.0 <= val{
                    false
                }
                else{
                    res.0 = l.0;
                    res.1 = r.1;
                    *is_valid_bst
                }
            }
            (None, Some(r))=>{
                if r.0 <= val{
                    false
                }
                else{
                    res.0 = val;
                    res.1 = r.1;
                    *is_valid_bst
                }
            }
            (Some(l), None)=>{
                if l.1 >= val{
                    false
                }
                else{
                    res.0 = l.0;
                    res.1 = val;
                    *is_valid_bst
                }
            }
            (None, None)=>{
                res.0 = val;
                res.1 = val;
                *is_valid_bst
            }
        };
        Some(res)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let input = Some(Rc::new(RefCell::new(TreeNode{
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let output = Solution::is_valid_bst(input);
        assert_eq!(true, output);
    }
    #[test]
    fn test2(){
        let input = Some(Rc::new(RefCell::new(TreeNode{
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 6,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let output = Solution::is_valid_bst(input);
        assert_eq!(false, output);
    }
    #[test]
    fn test3(){
        let input = Some(Rc::new(RefCell::new(TreeNode{
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 0,
                    left: None,
                    right: None,
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
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 6,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let output = Solution::is_valid_bst(input);
        assert_eq!(false, output);
    }
}