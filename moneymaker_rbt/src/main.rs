mod rbt;

use rbt::RedBlackTree;

fn main() {
    println!("=== Initializing Red-Black Tree ===");
    let mut tree = RedBlackTree::new();

    // Testing Insertions
    println!("Inserting nodes: 10, 20, 30, 15, 25, 5, 1");
    let elements = [10, 20, 30, 15, 25, 5, 1];
    for &el in &elements {
        tree.insert(el);
    }
    tree.print_tree();
    tree.print_inorder();

    // Testing Search
    println!("Search 15: {}", tree.search(15));
    println!("Search 100: {}", tree.search(100));

    // Testing Height & Count Leaves
    println!("Tree Height: {}", tree.height());
    println!("Leaf Count: {}", tree.count_leaves());

    // Testing Deletions
    println!("\n=== Deleting node 15 ===");
    tree.delete(15);
    tree.print_tree();

    println!("\n=== Deleting root (20) ===");
    tree.delete(20);
    tree.print_tree();

    println!("Final In-order:");
    tree.print_inorder();
}
