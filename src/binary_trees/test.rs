use crate::binary_trees::tree::Tree;

#[test]
pub fn btree_traversal_test() {
    let values = vec![1, 2, 3, 4, 5];
    let root = Tree::create_inorder(values);
    Tree::btree_traversal(&root);
}