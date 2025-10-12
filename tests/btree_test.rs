use rust_dsa::binary_trees::height_balance_check::BalanceWithHeight;
use rust_dsa::binary_trees::tree::Tree;

#[test]
pub fn btree_traversal_test() {
    let values = vec![1, 2, 3, 4, 5];
    let root = Tree::from_values(values);
    let mut result = vec![];
    Tree::preorder_collect(&root, &mut result);

    assert_eq!(result, vec![1, 2, 4, 5, 3]);
}

#[test]
pub fn btree_balance_test() {
    let values = vec![1, 2, 3, 4, 5];
    let root = Tree::from_values(values);
    let result = BalanceWithHeight::is_balanced(&root);

    assert_eq!(result, true);
}

#[test]
pub fn btree_symmetry_test() {
    let values = vec![1, 2, 2, 3, 4, 4, 3];
    let root = Tree::from_values(values);
    let result = rust_dsa::binary_trees::symmetry_check::is_symmetric(&root);

    assert_eq!(result, true);
}
