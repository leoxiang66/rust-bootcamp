pub struct BSTNode {
    value: i32,
    left: Option<Box<BSTNode>>,
    right: Option<Box<BSTNode>>,
}

pub struct BST {
    root: Option<Box<BSTNode>>,
}

impl BSTNode {
    fn new(value: i32) -> Self {
        BSTNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl BST {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn from_vec(values: Vec<i32>) -> Self {
        let mut bst = BST::new();
        for value in values {
            bst.insert(value);
        }
        bst
    }

    pub fn insert(&mut self, value: i32) {
        // TODO: 🦀Your task
    }

    pub fn search(&self, value: i32) -> bool {
        // TODO: 🦀Your task
    }

    pub fn min_value(&self) -> Option<i32> {
        // TODO: 🦀Your task
    }

    pub fn in_order(&self) -> Vec<i32> {
        // TODO: 🦀Your task
    }

    pub fn height(&self) -> usize {
        // TODO: 🦀Your task
    }
}
