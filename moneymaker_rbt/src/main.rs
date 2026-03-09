// mod rbt;

// use rbt::RedBlackTree;

fn main() {
    let mut tree = RedBlackTree::new();
    tree.insert(10);
    tree.insert(20);
    tree.insert(30);
    tree.print_tree();
}
