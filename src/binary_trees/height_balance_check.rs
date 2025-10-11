use crate::binary_trees::tree::Tree;

/*
Following are the conditions for a height-balanced binary tree:

- The difference between the heights of the left and the right subtree for any node is not more than one.
- The left subtree is balanced.
- The right subtree is balanced.

*/

pub struct BalanceWithHeight {
    balanced: bool,
    height: i32,
}

impl BalanceWithHeight {
    pub fn is_balanced(root: &Option<Box<Tree>>) -> bool {
        Self::check_balance(root).balanced
    }

    pub fn check_balance(root: &Option<Box<Tree>>) -> BalanceWithHeight {
        if root.is_none() {
            // base case
            return BalanceWithHeight {
                balanced: true,
                height: -1,
            };
        }

        let left_result = Self::check_balance(&root.as_ref().unwrap().left);
        if !left_result.balanced {
            return BalanceWithHeight {
                balanced: false,
                height: 0,
            };
        }
        let right_result = Self::check_balance(&root.as_ref().unwrap().right);
        if !right_result.balanced {
            return BalanceWithHeight {
                balanced: false,
                height: 0,
            };
        }

        let balanced = (left_result.height - right_result.height).abs() <= 1;
        let height = left_result.height.max(right_result.height) + 1;
        BalanceWithHeight { balanced, height }
    }
}
