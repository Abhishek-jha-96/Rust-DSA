pub struct Tree {
    value: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
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

    pub fn from_values(values: impl IntoIterator<Item = i32>) -> Option<Box<Tree>> {
        let mut root = None;
        for value in values {
            Self::insert(&mut root, value);
        }
        root
    }

    pub fn preorder_collect(root: &Option<Box<Self>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            result.push(node.value);
            Self::preorder_collect(&node.left, result);
            Self::preorder_collect(&node.right, result);
        }
    }
}
