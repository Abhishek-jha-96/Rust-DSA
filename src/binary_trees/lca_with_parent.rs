use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Node = Rc<RefCell<PTree>>;

pub struct PTree {
    pub value: i32,
    pub left: Option<Node>,
    pub right: Option<Node>,
    pub parent: Option<Weak<RefCell<PTree>>>,
}


impl PTree {
    fn new(value: i32) -> Node {
        Rc::new(RefCell::new(Self {
            value,
            left: None,
            right: None,
            parent: None,
        }))
    }

    pub fn create(arr: &[i32]) -> Option<Node> {
        if arr.is_empty() {
            return None;
        }

        // Create nodes for all values
        let nodes: Vec<Node> = arr.iter().map(|&v| PTree::new(v)).collect();

        // Link children & parents
        for i in 0..arr.len() {
            let left = 2 * i + 1;
            let right = 2 * i + 2;

            if left < arr.len() {
                nodes[i].borrow_mut().left = Some(nodes[left].clone());
                nodes[left].borrow_mut().parent = Some(Rc::downgrade(&nodes[i]));
            }

            if right < arr.len() {
                nodes[i].borrow_mut().right = Some(nodes[right].clone());
                nodes[right].borrow_mut().parent = Some(Rc::downgrade(&nodes[i]));
            }
        }

        Some(nodes[0].clone())
    }
}


pub fn lca_with_parent(node1: &Option<Node>, node2: &Option<Node>) -> Option<Node> {
    fn get_depth(node: &Option<Node>) -> i32 {
        let mut depth  = 0;
        let Some(node) = node else {
            return 0;
        };
        while (node.borrow().is_some()) {
            depth += 1;
            node = node.borrow().parent;
        }
        depth
    }
    


    
}
