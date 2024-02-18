struct Solution;
use crate::structure::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(x) = root.as_ref() {
            if Rc::ptr_eq(&x, &p.as_ref().unwrap()) || Rc::ptr_eq(&x, &q.as_ref().unwrap()) {
                return root;
            }
            let left = Self::lowest_common_ancestor(x.borrow().left.clone(), p.clone(), q.clone());
            let right = Self::lowest_common_ancestor(x.borrow().right.clone(), p.clone(), q.clone());
            if left.is_some() && right.is_some() {
                return root;
            }
            return if left.is_some() { left } else { right };
        }
        None
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let p = Some(Rc::new(RefCell::new(TreeNode{
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 8,
                left: None,
                right: None,
            }))),
        })));
        let res = Some(Rc::new(RefCell::new(TreeNode{
            val: 3,
            left: p.clone(),
            right: q.clone(),
        })));
        let input = res.clone();
        let output = Solution::lowest_common_ancestor(input.clone(), p.clone(), q.clone());
        assert!(Rc::ptr_eq(&res.as_ref().unwrap(), &output.as_ref().unwrap()));
    }
    #[test]
    fn test2(){
        let q = Some(Rc::new(RefCell::new(TreeNode{
            val: 4,
            left: None,
            right: None,
        })));
        let p = Some(Rc::new(RefCell::new(TreeNode{
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: q.clone(),
            }))),
        })));
        let res = p.clone();
        let input = Some(Rc::new(RefCell::new(TreeNode{
            val: 3,
            left: p.clone(),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 8,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let output = Solution::lowest_common_ancestor(input.clone(), p.clone(), q.clone());
        assert!(Rc::ptr_eq(&res.as_ref().unwrap(), &output.as_ref().unwrap()));
    }
    #[test]
    fn test3(){
        let q = Some(Rc::new(RefCell::new(TreeNode{
            val: 2,
            left: None,
            right: None,
        })));
        let p = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: q.clone(),
            right: None,
        })));
        let res = p.clone();
        let input = res.clone();
        let output = Solution::lowest_common_ancestor(input.clone(), p.clone(), q.clone());
        assert!(Rc::ptr_eq(&res.as_ref().unwrap(), &output.as_ref().unwrap()));
    }
}