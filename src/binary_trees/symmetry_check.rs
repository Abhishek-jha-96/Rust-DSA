use crate::binary_trees::tree::Tree;

pub fn check_symmetry(subtree_left: &Option<Box<Tree>>, subtree_right: &Option<Box<Tree>>) -> bool {
    if subtree_left.is_none() && subtree_right.is_none() {
        return true;
    } else if subtree_left.is_some() && subtree_right.is_some() {
        return (subtree_left.as_ref().unwrap().value == subtree_right.as_ref().unwrap().value)
            && check_symmetry(
                &subtree_left.as_ref().unwrap().left,
                &subtree_right.as_ref().unwrap().right,
            )
            && check_symmetry(
                &subtree_left.as_ref().unwrap().right,
                &subtree_right.as_ref().unwrap().left,
            );
    }

    false
}

pub fn is_symmetric(root: &Option<Box<Tree>>) -> bool {
    root.is_none() || check_symmetry(&root.as_ref().unwrap().left, &root.as_ref().unwrap().right)
}
