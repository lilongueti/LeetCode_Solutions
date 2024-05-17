// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let root_rc=root.unwrap();
        {
        let mut root_node=root_rc.borrow_mut();
        
        if let Some(left_rc)= &root_node.left
        {
            root_node.left=Solution::remove_leaf_nodes(root_node.left.take(), target);
        }
        if let Some(right_rc)= &root_node.right
        {
            root_node.right=Solution::remove_leaf_nodes(root_node.right.take(), target);
        }
        if root_node.val==target && root_node.left.is_none() && root_node.right.is_none()
        {
            
            return None;
        }
        }
        return Some(root_rc);
    }
    
}
