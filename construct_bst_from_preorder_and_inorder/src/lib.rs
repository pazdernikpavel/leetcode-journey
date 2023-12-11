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

struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        for preorder_element in 0..preorder.len() {
            let position_in_inorder = inorder
                .iter()
                .position(|&x| x == preorder[preorder_element]);
            match position_in_inorder {
                Some(position_in_inorder) => {
                    let left_inorder = &inorder[..position_in_inorder];
                    let right_inorder = &inorder[position_in_inorder + 1..];
                    let left_preorder = &preorder[1..=left_inorder.len()];
                    let right_preorder = &preorder[left_inorder.len() + 1..];
                    return Some(Rc::new(RefCell::new(TreeNode {
                        val: preorder[preorder_element],
                        left: Solution::build_tree(left_preorder.to_vec(), left_inorder.to_vec()),
                        right: Solution::build_tree(
                            right_preorder.to_vec(),
                            right_inorder.to_vec(),
                        ),
                    })));
                }
                None => continue,
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        assert_eq!(Solution::build_tree(preorder, inorder), expected);
    }
}
