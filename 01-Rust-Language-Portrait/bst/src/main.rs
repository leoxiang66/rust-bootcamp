mod bst;

fn main() {
    let values = vec![10, 5, 15, 3, 7, 12, 18];
    let mut tree = bst::BST::from_vec(values);

    println!("Search 7: {}", tree.search(7));
    println!("Search 20: {}", tree.search(20));

    tree.insert(20);
    println!("Search 20: {}", tree.search(20));

    println!("Height of tree: {}", tree.height());

    match tree.min_value() {
        Some(value) => println!("Min value: {}", value),
        None => println!("No min tree is empty"),
    }

    let in_order = tree.in_order();
    println!("In-order traversal: {:?}", in_order);
}

#[cfg(test)]
mod tests {
    use super::bst::BST;

    #[test]
    fn test_insert_and_search() {
        let mut tree = BST::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        assert!(tree.search(10));
        assert!(tree.search(5));
        assert!(tree.search(15));
        assert!(!tree.search(20));
    }

    #[test]
    fn test_height() {
        let mut tree = BST::new();
        assert_eq!(tree.height(), 0);
        tree.insert(10);
        assert_eq!(tree.height(), 1);
        tree.insert(5);
        tree.insert(15);
        assert_eq!(tree.height(), 2);
        tree.insert(3);
        tree.insert(7);
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn test_in_order_traversal() {
        let values = vec![10, 5, 15, 3, 7, 12, 18];
        let tree = BST::from_vec(values);
        let result = tree.in_order();
        assert_eq!(result, vec![3, 5, 7, 10, 12, 15, 18]);
    }

    #[test]
    fn test_min_value() {
        let values = vec![10, 5, 15, 3, 7, 12, 18];
        let tree = BST::from_vec(values);
        assert_eq!(tree.min_value(), Some(3));
    }

    #[test]
    fn test_insert_duplicates() {
        let mut tree = BST::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(10);
        tree.insert(15);
        let result = tree.in_order();
        assert_eq!(result, vec![5, 10, 15]);
    }

    #[test]
    fn test_empty_tree() {
        let tree: BST = BST::new();
        assert_eq!(tree.height(), 0);
        assert_eq!(tree.in_order(), Vec::<i32>::new());
        assert_eq!(tree.min_value(), None);
        assert!(!tree.search(10));
    }
}
