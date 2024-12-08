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
        self.root = Self::insert_node(self.root.take(), value);
    }

    fn insert_node(node: Option<Box<BSTNode>>, value: i32) -> Option<Box<BSTNode>> {
        match node {
            Some(mut current) => {
                if value < current.value {
                    current.left = Self::insert_node(current.left.take(), value);
                } else if value > current.value {
                    current.right = Self::insert_node(current.right.take(), value);
                }
                Some(current)
            }
            None => Some(Box::new(BSTNode::new(value))),
        }
    }

    pub fn search(&self, value: i32) -> bool {
        Self::search_node(&self.root, value)
    }

    fn search_node(node: &Option<Box<BSTNode>>, value: i32) -> bool {
        match node {
            Some(current) => {
                if value == current.value {
                    true
                } else if value < current.value {
                    Self::search_node(&current.left, value)
                } else {
                    Self::search_node(&current.right, value)
                }
            }
            None => false,
        }
    }

    pub fn min_value(&self) -> Option<i32> {
        Self::find_min(&self.root)
    }

    fn find_min(node: &Option<Box<BSTNode>>) -> Option<i32> {
        let mut current = node;
        while let Some(n) = current {
            if n.left.is_none() {
                return Some(n.value);
            }
            current = &n.left;
        }
        None
    }

    pub fn in_order(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::in_order_traverse(&self.root, &mut result);
        result
    }

    fn in_order_traverse(node: &Option<Box<BSTNode>>, result: &mut Vec<i32>) {
        if let Some(current) = node {
            Self::in_order_traverse(&current.left, result);
            result.push(current.value);
            Self::in_order_traverse(&current.right, result);
        }
    }

    pub fn height(&self) -> usize {
        Self::calculate_height(&self.root)
    }

    fn calculate_height(node: &Option<Box<BSTNode>>) -> usize {
        match node {
            Some(current) => {
                1 + std::cmp::max(
                    Self::calculate_height(&current.left),
                    Self::calculate_height(&current.right),
                )
            }
            None => 0,
        }
    }
}
