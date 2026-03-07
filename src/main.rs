// =============================================================================
// main.rs — Command Line Interface
// =============================================================================
//
// This is the entry point for the program. It provides an interactive menu
// that lets the user test both the Red-Black Tree and AVL Tree, and also
// run the benchmarks.
// =============================================================================

mod avl;
mod benchmark;
mod rbt;

use avl::AVLTree;
use rbt::RedBlackTree;
use std::io::{self, BufRead, Write};

fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║       ECE 421 Project 2 — Trees, Trees, More Trees   ║");
    println!("║           Red-Black Tree & AVL Tree in Rust          ║");
    println!("╚══════════════════════════════════════════════════════╝");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        println!("\n┌─── Main Menu ───────────────────────────────┐");
        println!("│  1. Red-Black Tree operations                │");
        println!("│  2. AVL Tree operations                      │");
        println!("│  3. Run benchmarks (RBT vs AVL)              │");
        println!("│  4. Run demo (auto-insert & show both trees) │");
        println!("│  q. Quit                                     │");
        println!("└─────────────────────────────────────────────┘");
        print!("Choice: ");
        io::stdout().flush().unwrap();

        let input = match lines.next() {
            Some(Ok(line)) => line.trim().to_string(),
            _ => break,
        };

        match input.as_str() {
            "1" => run_rbt_menu(&mut lines),
            "2" => run_avl_menu(&mut lines),
            "3" => benchmark::run_benchmarks(),
            "4" => run_demo(),
            "q" | "Q" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown option. Please enter 1, 2, 3, 4, or q."),
        }
    }
}


fn run_rbt_menu(lines: &mut impl Iterator<Item = io::Result<String>>) {
    let mut tree = RedBlackTree::new();
    println!("\n[ Red-Black Tree — empty tree created ]");

    loop {
        println!("\n┌─── RBT Menu ─────────────────────────────────┐");
        println!("│  i <n>   Insert value n                      │");
        println!("│  d <n>   Delete value n                      │");
        println!("│  s <n>   Search for value n                  │");
        println!("│  l       Count leaves                        │");
        println!("│  h       Get height                          │");
        println!("│  e       Check if empty                      │");
        println!("│  o       Print in-order traversal            │");
        println!("│  p       Print tree structure                │");
        println!("│  b       Back to main menu                   │");
        println!("└──────────────────────────────────────────────┘");
        print!("RBT> ");
        io::stdout().flush().unwrap();

        let input = match lines.next() {
            Some(Ok(line)) => line.trim().to_string(),
            _ => break,
        };

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        match parts[0] {
            "i" => {
                if let Some(n) = parse_u32(parts.get(1)) {
                    tree.insert(n);
                    println!("Inserted {}.", n);
                } else {
                    println!("Usage: i <number>");
                }
            }
            "d" => {
                if let Some(n) = parse_u32(parts.get(1)) {
                    if tree.delete(n) {
                        println!("Deleted {}.", n);
                    } else {
                        println!("{} not found in tree.", n);
                    }
                } else {
                    println!("Usage: d <number>");
                }
            }
            "s" => {
                if let Some(n) = parse_u32(parts.get(1)) {
                    if tree.search(n) {
                        println!("{} FOUND in tree.", n);
                    } else {
                        println!("{} NOT found in tree.", n);
                    }
                } else {
                    println!("Usage: s <number>");
                }
            }
            "l" => println!("Leaf count: {}", tree.count_leaves()),
            "h" => println!("Height: {}", tree.height()),
            "e" => println!("Is empty: {}", tree.is_empty()),
            "o" => tree.print_inorder(),
            "p" => tree.print_tree(),
            "b" => break,
            _ => println!("Unknown command."),
        }
    }
}


fn run_avl_menu(lines: &mut impl Iterator<Item = io::Result<String>>) {
    let mut tree = AVLTree::new();
    println!("\n[ AVL Tree — empty tree created ]");

    loop {
        println!("\n┌─── AVL Menu ─────────────────────────────────┐");
        println!("│  i <n>   Insert value n                      │");
        println!("│  d <n>   Delete value n                      │");
        println!("│  s <n>   Search for value n                  │");
        println!("│  l       Count leaves                        │");
        println!("│  h       Get height                          │");
        println!("│  e       Check if empty                      │");
        println!("│  o       Print in-order traversal            │");
        println!("│  p       Print tree structure                │");
        println!("│  b       Back to main menu                   │");
        println!("└──────────────────────────────────────────────┘");
        print!("AVL> ");
        io::stdout().flush().unwrap();

        let input = match lines.next() {
            Some(Ok(line)) => line.trim().to_string(),
            _ => break,
        };

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        match parts[0] {
            "i" => {
                if let Some(n) = parse_u32(parts.get(1)) {
                    tree.insert(n);
                    println!("Inserted {}.", n);
                } else {
                    println!("Usage: i <number>");
                }
            }
            "d" => {
                if let Some(n) = parse_u32(parts.get(1)) {
                    if tree.delete(n) {
                        println!("Deleted {}.", n);
                    } else {
                        println!("{} not found in tree.", n);
                    }
                } else {
                    println!("Usage: d <number>");
                }
            }
            "s" => {
                if let Some(n) = parse_u32(parts.get(1)) {
                    if tree.search(n) {
                        println!("{} FOUND in tree.", n);
                    } else {
                        println!("{} NOT found in tree.", n);
                    }
                } else {
                    println!("Usage: s <number>");
                }
            }
            "l" => println!("Leaf count: {}", tree.count_leaves()),
            "h" => println!("Height: {}", tree.height()),
            "e" => println!("Is empty: {}", tree.is_empty()),
            "o" => tree.print_inorder(),
            "p" => tree.print_tree(),
            "b" => break,
            _ => println!("Unknown command."),
        }
    }
}


fn run_demo() {
    println!("\n══════════════════════════════════════════════════");
    println!("  DEMO: Inserting [10, 20, 30, 15, 25, 5, 1, 7]");
    println!("══════════════════════════════════════════════════");

    let values = [10u32, 20, 30, 15, 25, 5, 1, 7];

    println!("\n─── Red-Black Tree ───────────────────────────────");
    let mut rbt = RedBlackTree::new();
    for &v in &values {
        rbt.insert(v);
    }
    rbt.print_tree();
    rbt.print_inorder();
    println!("Height:       {}", rbt.height());
    println!("Leaf count:   {}", rbt.count_leaves());
    println!("Is empty:     {}", rbt.is_empty());

    println!("\nDeleting 20 from RBT...");
    rbt.delete(20);
    rbt.print_tree();
    rbt.print_inorder();

    println!("\n─── AVL Tree ──────────────────────────────────────");
    let mut avl = AVLTree::new();
    for &v in &values {
        avl.insert(v);
    }
    avl.print_tree();
    avl.print_inorder();
    println!("Height:       {}", avl.height());
    println!("Leaf count:   {}", avl.count_leaves());
    println!("Is empty:     {}", avl.is_empty());

    println!("\nDeleting 20 from AVL...");
    avl.delete(20);
    avl.print_tree();
    avl.print_inorder();
}


fn parse_u32(s: Option<&&str>) -> Option<u32> {
    s?.trim().parse().ok()
}
