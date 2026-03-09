//! # Project 2: Red-Black Tree & AVL Tree Implementation
//!
//! This project provides high-performance Rust implementations of two self-balancing binary search trees:
//! **Red-Black Tree (RBT)** and **AVL Tree**.
//!
//! ## 1. Design Document
//!
//! ### Extensions and Enhancements
//! - **Interactive Command-Line Interface (CLI):** A robust, menu-driven interface allows users to perform real-time tree operations and visualize the tree structures.
//! - **Rich Visualizations:** Unicode-based tree printing demonstrates the hierarchical structure and balancing properties (including color tags for RBT and balance factors for AVL).
//! - **Automated Benchmarking Suite:** A dedicated module that measures and compares performance across five different tree sizes, providing a detailed statistical analysis of operation times.
//! - **Integrated Demo Mode:** A "run-and-see" feature that automatically populates both trees with complex test cases and displays their final state and performance metrics.
//!
//! ### Design Questions
//! 1. **What does a red-black tree provide that cannot be accomplished with ordinary binary search trees?**
//!    Red-black trees guarantee a logarithmic upper bound for key operations (O(log n)) by maintaining a balanced height. Naive BSTs can degenerate to O(n) "linked lists" when data is inserted in sorted or semi-sorted order, leading to performance bottlenecks.
//!
//! 2. **Do you need to apply any kind of error handling in your system?**
//!    Yes. We utilize Rust's `Option<T>` for pointers (`NodeRef`) to handle potentially null children or parents safely. The CLI also includes input validation (e.g., `parse_u32`) to prevent invalid data or malformed commands from crashing the program.
//!
//! 3. **What components do the Red-black tree and AVL tree have in common?**
//!    Both trees share the fundamental binary search tree structure (ordered keys) and the mechanism of **Tree Rotations** (left and right) to maintain balance. They both use smart pointers (`Rc<RefCell<Node>>`) for safe, shared, mutable ownership within the tree.
//!
//! 4. **How do we construct our design to "allow it to be efficiently and effectively extended"?**
//!    By leveraging Rust's module system and clear structural separation. The core logic for each tree is encapsulated in private modules and exposed through a clean, unified public API. This modularity allows for the integration of further data structures (like 2-3-4 or B-trees) by following the same trait-like interface without modifying the existing library code.
//!
//! ### Benchmarking Questions
//! 1. **Which data structure is more efficient?**
//!    Based on the benchmark results, **AVL Trees** are generally more efficient for **SEARCHES** because they are more strictly balanced, resulting in shorter average search paths. **Red-Black Trees** are typically more efficient for **INSERTS** and **DELETIONS** as they require fewer rebalancing rotations on average compared to AVL trees.
//!
//! 2. **Do you think we need to accommodate other test cases?**
//!    Yes. While the current "sequential insert" test case is a worst-case for height, we should also test **randomly distributed data** (average case) and **partially sorted data** (common in real-world scenarios). Additionally, benchmarking the **deletion** of large sets of nodes would provide more insight into the rebalancing overhead.
//!
//! 3. **Should we include additional data structures in the benchmarking as a baseline?**
//!    Including a **naive Binary Search Tree** as a baseline would be highly informative. It would demonstrate the dramatic performance loss of unbalanced trees on sequential data (O(n)) compared to the sustained performance of RBT and AVL (O(log n)).
//!
//! ### Known Errors, Faults, and Limitations
//! - **RBT Deletion Rebalancing:** Although implemented as per the standard CLRS algorithm, the logic is highly complex; while functional, there may be subtle edge cases in balance restoration if non-unique keys were used (though mitigated by the `insert` logic).
//! - **UI Alignment:** For very large terminal widths or extremely deep trees, the `print_tree` output may wrap or become difficult to read.
//! - **Benchmarking Variance:** Operation times are measured in microseconds and averaged over 3 runs; results may still show minor fluctuations depending on system load.
//!
//! ## 2. User Manual
//!
//! ### Running the Application
//! To start the interactive session, ensure you have Rust/Cargo installed and run:
//! ```bash
//! cargo run
//! ```
//!
//! ### Commands and Navigation
//! Once inside the program, navigate using the main menu:
//! 1. **Red-Black Tree operations:** Opens the RBT submenu.
//!    - `i <n>`: Insert value `n`
//!    - `d <n>`: Delete value `n`
//!    - `s <n>`: Search for value `n`
//!    - `p`: Print visual tree structure
//!    - `o`: Print in-order traversal
//!    - `b`: Back to main menu
//! 2. **AVL Tree operations:** Opens the AVL submenu (same commands as RBT).
//! 3. **Run benchmarks:** Executes the performance suite and displays a summary table.
//! 4. **Run demo:** Automatically performs a predefined set of operations to demonstrate the functionality.
//! `q`: Exit the program.

// =============================================================================
// main.rs — Command Line Interface
// =============================================================================
//
// This is the entry point for the program. It provides an interactive menu
// that lets the user test both the Red-Black Tree and AVL Tree, and also
// run the benchmarks.
// =============================================================================

mod advanced;
mod benchmark;

impl advanced::TreeOps for RedBlackTree {
    fn insert(&mut self, v: u32) {
        RedBlackTree::insert(self, v);
    }
    fn delete(&mut self, v: u32) -> bool {
        RedBlackTree::delete(self, v)
    }
    fn search(&self, v: u32) -> bool {
        RedBlackTree::search(self, v)
    }
    fn count_leaves(&self) -> usize {
        RedBlackTree::count_leaves(self)
    }
    fn height(&self) -> usize {
        RedBlackTree::height(self)
    }
    fn is_empty(&self) -> bool {
        RedBlackTree::is_empty(self)
    }
    fn print_inorder(&self) {
        RedBlackTree::print_inorder(self);
    }
    fn print_tree(&self) {
        RedBlackTree::print_tree(self);
    }
}

impl advanced::TreeOps for AVLTree {
    fn insert(&mut self, v: u32) {
        AVLTree::insert(self, v);
    }
    fn delete(&mut self, v: u32) -> bool {
        AVLTree::delete(self, v)
    }
    fn search(&self, v: u32) -> bool {
        AVLTree::search(self, v)
    }
    fn count_leaves(&self) -> usize {
        AVLTree::count_leaves(self)
    }
    fn height(&self) -> usize {
        AVLTree::height(self)
    }
    fn is_empty(&self) -> bool {
        AVLTree::is_empty(self)
    }
    fn print_inorder(&self) {
        AVLTree::print_inorder(self);
    }
    fn print_tree(&self) {
        AVLTree::print_tree(self);
    }
}

use moneymaker_avl::AVLTree;
use moneymaker_rbt::RedBlackTree;
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
        println!("│  5. Advanced RBT                             │");
        println!("│  6. Advanced AVL                             │");
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
            "5" => run_rbt_advanced(&mut lines),
            "6" => run_avl_advanced(&mut lines),
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

fn run_rbt_advanced(lines: &mut impl Iterator<Item = io::Result<String>>) {
    let mut tree = RedBlackTree::new();
    advanced::print_lang_help();

    println!("\n[ Advanced RBT — empty tree created ]");
    println!("  Enter multi-command expressions, e.g:");
    println!("    i rang 1 10 h p");
    println!("    r i rand 1 100 5 h t 3");
    println!("    b to go back, q to quit");

    loop {
        print!("RBT+> ");
        io::stdout().flush().unwrap();
        let input = match lines.next() {
            Some(Ok(line)) => line.trim().to_string(),
            _ => break,
        };
        if input.is_empty() {
            continue;
        }
        if input == "?" {
            advanced::print_lang_help();
            continue;
        }
        match advanced::run_line(&input, &mut tree) {
            Ok(advanced::ExecResult::Continue) => {}
            Ok(advanced::ExecResult::Back) | Ok(advanced::ExecResult::Quit) => break,
            Err(e) => println!("  ✗ {}", e),
        }
    }
}

fn run_avl_advanced(lines: &mut impl Iterator<Item = io::Result<String>>) {
    let mut tree = AVLTree::new();
    advanced::print_lang_help();

    println!("\n[ Advanced AVL — empty tree created ]");
    println!("  Enter multi-command expressions, e.g:");
    println!("    i rang 1 10 h p");
    println!("    r i rand 1 100 5 h t 3");
    println!("    b to go back, q to quit");

    loop {
        print!("AVL+> ");
        io::stdout().flush().unwrap();
        let input = match lines.next() {
            Some(Ok(line)) => line.trim().to_string(),
            _ => break,
        };
        if input.is_empty() {
            continue;
        }
        if input == "?" {
            advanced::print_lang_help();
            continue;
        }
        match advanced::run_line(&input, &mut tree) {
            Ok(advanced::ExecResult::Continue) => {}
            Ok(advanced::ExecResult::Back) | Ok(advanced::ExecResult::Quit) => break,
            Err(e) => println!("  ✗ {}", e),
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
