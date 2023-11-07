use std::cell::RefCell;
use std::rc::Rc;

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
            right: None,
        }
    }
}

struct Solution {}
impl Solution {
    pub fn traverse(values: &mut Vec<i32>, node: Option<&Rc<RefCell<TreeNode>>>) {
        if let Some(node_unwrapped) = node {
            Solution::traverse(values, node_unwrapped.borrow().left.as_ref());
            values.push(node_unwrapped.borrow().val);
            Solution::traverse(values, node_unwrapped.borrow().right.as_ref());
        }
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        Solution::traverse(&mut result, root.as_ref());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));

        println!("{:?}", Solution::inorder_traversal(input));
    }
}
