#[derive(Debug)]
pub struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: i32) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(value)));
            return;
        }

        let mut current = self.root.as_mut();
        while let Some(node) = current {
            match (&mut node.left,&mut node.right) {
                (Some(_), Some(_)) => {
                    if value < node.value {
                        current = node.left.as_mut();
                    } else {
                        current = node.right.as_mut();
                    }
                }
                (Some(_), None) => {
                    if value < node.value {
                        current = node.left.as_mut();
                    } else {
                        node.right = Some(Box::new(Node::new(value)));
                        current = None;
                    }
                }
                (None, Some(_)) => {
                    if value > node.value {
                        current = node.right.as_mut();
                    } else {
                        node.left = Some(Box::new(Node::new(value)));
                        current = None;
                    }
                }
                (None, None) => {
                    if value < node.value {
                        node.left = Some(Box::new(Node::new(value)));
                    } else {
                        node.right = Some(Box::new(Node::new(value)));
                    }
                    current = None;
                }
            }
        }
    }

    pub fn lookup(&self, value: i32) -> bool {
        let mut current = self.root.as_ref();
        while let Some(node) = current {
            if value == node.value {
                return true;
            } else if value < node.value {
                current = node.left.as_ref();
            } else {
                current = node.right.as_ref();
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_perform_operations() {
        let mut bst = BinarySearchTree::new();
        bst.insert(9);
        bst.insert(4);
        bst.insert(6);
        bst.insert(20);
        bst.insert(170);
        bst.insert(15);
        bst.insert(1);
        println!("{:#?}", bst);
    }

    #[test]
    fn should_perform_lookup() {
        let mut bst = BinarySearchTree::new();
        bst.insert(9);
        bst.insert(4);
        bst.insert(6);
        bst.insert(20);
        bst.insert(170);
        bst.insert(15);
        bst.insert(1);

        assert_eq!(bst.lookup(9), true);
        assert_eq!(bst.lookup(4), true);
        assert_eq!(bst.lookup(170), true);
        assert_eq!(bst.lookup(1), true);
        assert_eq!(bst.lookup(99), false);
        assert_eq!(bst.lookup(2), false);
        assert_eq!(bst.lookup(171), false);
        assert_eq!(bst.lookup(45), false);
    }
}
