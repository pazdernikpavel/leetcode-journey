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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(node) = root {
            let mut borrowed_node = node.borrow_mut();
            Solution::range_sum_bst(borrowed_node.left.take(), low, high)
                + Solution::range_sum_bst(borrowed_node.right.take(), low, high)
                + {
                    if borrowed_node.val >= low && borrowed_node.val <= high {
                        borrowed_node.val
                    } else {
                        0
                    }
                }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test() {
        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 18,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        let result = Solution::range_sum_bst(input, 7, 15);
        assert_eq!(result, 32);
    }
}
