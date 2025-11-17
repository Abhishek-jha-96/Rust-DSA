use crate::binary_trees::tree::Tree;


pub fn lca_helper(
    root: &Option<Box<Tree>>,
    node1: i32,
    node2: i32,
) -> (i32, Option<i32>) {
    // If empty subtree
    if root.is_none() {
        return (0, None);
    }

    let node = root.as_ref().unwrap();

    // Recursively call left and right
    let (left_count, left_lca) = lca_helper(&node.left, node1, node2);
    if left_lca.is_some() {
        return (2, left_lca);      // Already found in left subtree
    }

    let (right_count, right_lca) = lca_helper(&node.right, node1, node2);
    if right_lca.is_some() {
        return (2, right_lca);     // Already found in right subtree
    }

    // Count matches in this subtree
    let mut count = left_count + right_count;
    if node.value == node1 || node.value == node2 {
        count += 1;
    }

    // If both found in this subtree → current node is LCA
    if count == 2 {
        return (2, Some(node.value));
    }

    (count, None)
}

pub fn lowest_common_ancestor(root: &Option<Box<Tree>>, node1: &Option<Box<Tree>>, node2: &Option<Box<Tree>>) -> (i32, Option<i32>) {
    let val1 = node1.as_ref().unwrap().value;
    let val2 = node2.as_ref().unwrap().value;

    lca_helper(root, val1, val2)

}