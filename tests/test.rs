use rust_dsa::binary_trees::tree::Tree;

#[test]
pub fn btree_traversal_test() {
    let values = vec![1, 2, 3, 4, 5];
    let root = Tree::from_values(values);
    let mut result = vec![];
    Tree::preorder_collect(&root, &mut result);

    assert_eq!(result, vec![1, 2, 4, 5, 3]);
}
