use std::cell::RefCell;
use std::cmp::Ordering;
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
    pub fn find_min(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_node) = root {
            let mut current = root_node.clone();
            while let Some(left) = &current.clone().borrow().left {
                current = left.clone()
            }
            Some(current.clone())
        } else {
            None
        }
    }

    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match &root {
            None => return None,
            Some(node) => {
                let borrowed_node = &mut node.borrow_mut();
                match borrowed_node.val.cmp(&key) {
                    Ordering::Less => {
                        borrowed_node.right = Solution::delete_node(borrowed_node.right.take(), key)
                    }
                    Ordering::Greater => {
                        borrowed_node.left = Solution::delete_node(borrowed_node.left.take(), key)
                    }
                    Ordering::Equal => match (&borrowed_node.left, &borrowed_node.right) {
                        (None, None) => return None,
                        (Some(_), None) => return borrowed_node.left.clone(),
                        (None, Some(_)) => return borrowed_node.right.clone(),
                        (Some(_), Some(_)) => {
                            let min = Solution::find_min(&borrowed_node.right.clone());
                            let min_val = min.clone().unwrap().borrow().val;
                            borrowed_node.val = min_val;
                            borrowed_node.right =
                                Solution::delete_node(borrowed_node.right.take(), min_val)
                        }
                    },
                }
            }
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test() {
        let mut input = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
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
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        })));

        input = Solution::delete_node(input, 3);
        println!("{:?}", input);
    }

    #[test]
    fn test2() {
        let mut input = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        println!("{:?}", input);
        input = Solution::delete_node(input, 3);
        println!("{:?}", input);
    }
}
