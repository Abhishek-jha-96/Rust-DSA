#[allow(dead_code)]
pub struct Tree {
    value: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    pub fn new(value: i32) -> Tree {
        Tree {
            value,
            left: None,
            right: None,
        }
    }
}
