pub struct Tree {
    pub value: i32,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

impl Tree {
    pub fn new(value: i32) -> Self {
        Tree {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(root: &mut Option<Box<Tree>>, value: i32) {
        match root {
            Some(node) => {
                if node.left.is_none() {
                    node.left = Some(Box::new(Tree::new(value)));
                } else if node.right.is_none() {
                    node.right = Some(Box::new(Tree::new(value)));
                } else {
                    Self::insert(&mut node.left, value);
                }
            }
            None => {
                *root = Some(Box::new(Tree::new(value)));
            }
        }
    }

    pub fn from_values(values: Vec<i32>) -> Option<Box<Tree>> {
        use std::collections::VecDeque;

        if values.is_empty() {
            return None;
        }

        let root = Box::new(Tree::new(values[0]));
        let mut queue = VecDeque::new();
        queue.push_back(root.as_ref() as *const Tree as *mut Tree);

        let mut i = 1;
        while i < values.len() {
            let node_ptr = queue.pop_front().unwrap();
            unsafe {
                if i < values.len() {
                    let left = Box::new(Tree::new(values[i]));
                    (*node_ptr).left = Some(left);
                    queue.push_back((*node_ptr).left.as_mut().unwrap().as_mut());
                    i += 1;
                }
                if i < values.len() {
                    let right = Box::new(Tree::new(values[i]));
                    (*node_ptr).right = Some(right);
                    queue.push_back((*node_ptr).right.as_mut().unwrap().as_mut());
                    i += 1;
                }
            }
        }

        Some(root)
    }

    pub fn preorder_collect(root: &Option<Box<Self>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            result.push(node.value);
            Self::preorder_collect(&node.left, result);
            Self::preorder_collect(&node.right, result);
        }
    }
}
