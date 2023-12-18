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

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_valid_bst(root: OptNode) -> bool {
        Self::is_valid(&root, i32::MIN as i64 - 1, i32::MAX as i64 + 1)
    }

    fn is_valid(node: &OptNode, gt: i64, lt: i64) -> bool {
        match node.as_ref() {
            None => true,
            Some(n) => {
                let b = n.borrow();
                (b.val as i64) > gt
                    && (b.val as i64) < lt
                    && Self::is_valid(&b.left, gt, b.val as i64)
                    && Self::is_valid(&b.right, b.val as i64, lt)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test1() {
        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        assert!(Solution::is_valid_bst(input));
    }

    #[test]
    fn test2() {
        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert!(!Solution::is_valid_bst(input));
    }
}
