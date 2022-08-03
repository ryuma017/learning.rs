use binary_search_tree::Node;

fn main() {
    let mut root = Node::new(25);
    for n in 0..5 {
        root.insert(n * 7);
        root.insert(n * 3);
    }
    println!("{:#?}", root);
}