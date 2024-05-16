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
    fn evaluate_or(root_rc:Rc<RefCell<TreeNode>>)->bool
    {
        let root= root_rc.borrow();
        let left=root.left.as_ref().unwrap();;
        if(Solution::evaluate_node(left.clone())){
            return true;
        }
        let right=root.right.as_ref().unwrap();
        if(Solution::evaluate_node(right.clone())){
            return true;
        }
        return false;
    }
    fn evaluate_and(root_rc:Rc<RefCell<TreeNode>>)->bool
    {
        let root=root_rc.borrow();
        let left=root.left.as_ref().unwrap();
        let right=root.right.as_ref().unwrap();
        return Solution::evaluate_node(left.clone()) && Solution::evaluate_node(right.clone());
    }
    fn evaluate_node(root:Rc<RefCell<TreeNode>>)->bool
    {
        let mut root_node=root.borrow_mut();
        if root_node.val==0{
            return false;
        }
        
        if root_node.val==1{
            return true;
        }
        if root_node.val==2{
            std::mem::drop(root_node);
            return Solution::evaluate_or(root.clone());
        }
        if root_node.val==3{
            std::mem::drop(root_node);
            return Solution::evaluate_and(root.clone());
        }
        
        return true;
    }
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        
        let root_rc=root.unwrap();
        return Solution::evaluate_node(root_rc.clone());
    }
    
}
