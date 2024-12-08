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
