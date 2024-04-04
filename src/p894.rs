use std::rc::Rc;
use std::cell::RefCell;
use crate::structure::TreeNode;
#[allow(unused)]
struct Solution;
impl Solution {
    #[allow(unused)]
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 1{
            let tree = Rc::new(RefCell::new(TreeNode::new(0)));
            return vec![Some(tree)];
        }
        if n == 3{
            let tree = Rc::new(RefCell::new(TreeNode{
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 0,
                    left: None,
                    right: None,
                }))),
            }));
            return vec![Some(tree)];
        }
        let m = n - 1;
        let mut res = Vec::new();
        for i in 1..m{
            for left_tree in Self::all_possible_fbt(i) {
                for right_tree in Self::all_possible_fbt(m - i) {
                    let root = Rc::new(RefCell::new(TreeNode::new(0)));
                    root.borrow_mut().left = left_tree.clone();
                    root.borrow_mut().right = right_tree.clone();
                    res.push(Some(root));
                }
            }
        }
        return res;
    }
}