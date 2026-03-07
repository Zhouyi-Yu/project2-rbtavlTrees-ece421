// =============================================================================
// avl.rs — AVL Tree
// =============================================================================
//
// WHAT IS AN AVL TREE?
// An AVL tree (Adelson-Velsky and Landis, 1962) is a self-balancing binary
// search tree where the HEIGHT DIFFERENCE between the left and right subtree
// of ANY node is at most 1. This difference is called the "balance factor".
//
// BALANCE FACTOR = height(left subtree) - height(right subtree)
// Allowed values: -1, 0, or +1
//
// When an insert or delete makes the balance factor go outside [-1, 1],
// we perform rotations to restore balance.
//
// FOUR ROTATION CASES:
//   1. Left-Left (LL): Right rotation around the unbalanced node.
//   2. Right-Right (RR): Left rotation around the unbalanced node.
//   3. Left-Right (LR): Left rotation on left child, then right rotation.
//   4. Right-Left (RL): Right rotation on right child, then left rotation.
//
// AVL vs Red-Black Tree:
//   AVL trees are MORE strictly balanced (height difference ≤ 1 vs ~2x).
//   This makes AVL trees faster for SEARCHES but slightly slower for
//   INSERT/DELETE because more rotations may be needed.
//
// DATA STRUCTURE:
// We use Option<Rc<RefCell<Node>>> for safe shared mutable references,
// as required by the project specification.
// =============================================================================

use std::cell::RefCell;
use std::fmt::{self, Display};
use std::rc::Rc;

pub type AVLLink = Option<Rc<RefCell<AVLNode>>>;

pub struct AVLNode {
    pub key: u32,
    pub height: i32,
    pub left: AVLLink,
    pub right: AVLLink,
}

impl AVLNode {
    pub fn new(key: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(AVLNode {
            key,
            height: 1,
            left: None,
            right: None,
        }))
    }
}

pub struct AVLTree {
    root: AVLLink,
}

fn node_height(link: &AVLLink) -> i32 {
    match link {
        None => 0,
        Some(n) => n.borrow().height,
    }
}

fn update_height(node: &Rc<RefCell<AVLNode>>) {
    let lh = node_height(&node.borrow().left);
    let rh = node_height(&node.borrow().right);
    node.borrow_mut().height = 1 + lh.max(rh);
}

fn balance_factor(node: &Rc<RefCell<AVLNode>>) -> i32 {
    let lh = node_height(&node.borrow().left);
    let rh = node_height(&node.borrow().right);
    lh - rh
}

fn clone_link(link: &AVLLink) -> AVLLink {
    link.as_ref().map(Rc::clone)
}

fn rotate_right(y: Rc<RefCell<AVLNode>>) -> Rc<RefCell<AVLNode>> {
    let x = clone_link(&y.borrow().left).expect("rotate_right: left child must exist");

    let x_right = clone_link(&x.borrow().right);
    y.borrow_mut().left = x_right;

    x.borrow_mut().right = Some(Rc::clone(&y));

    update_height(&y);
    update_height(&x);

    x
}

fn rotate_left(x: Rc<RefCell<AVLNode>>) -> Rc<RefCell<AVLNode>> {
    let y = clone_link(&x.borrow().right).expect("rotate_left: right child must exist");

    let y_left = clone_link(&y.borrow().left);
    x.borrow_mut().right = y_left;

    y.borrow_mut().left = Some(Rc::clone(&x));

    update_height(&x);
    update_height(&y);

    y
}

fn rebalance(node: Rc<RefCell<AVLNode>>) -> Rc<RefCell<AVLNode>> {
    update_height(&node);
    let bf = balance_factor(&node);

    if bf > 1 {
        let left = clone_link(&node.borrow().left).unwrap();
        if balance_factor(&left) < 0 {
            let new_left = rotate_left(left);
            node.borrow_mut().left = Some(new_left);
        }
        return rotate_right(node);
    }

    if bf < -1 {
        let right = clone_link(&node.borrow().right).unwrap();
        if balance_factor(&right) > 0 {
            let new_right = rotate_right(right);
            node.borrow_mut().right = Some(new_right);
        }
        return rotate_left(node);
    }

    node
}

impl AVLTree {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn insert(&mut self, key: u32) {
        self.root = Self::insert_rec(clone_link(&self.root), key);
    }

    fn insert_rec(node: AVLLink, key: u32) -> AVLLink {
        match node {
            None => Some(AVLNode::new(key)),
            Some(n) => {
                let node_key = n.borrow().key;
                if key < node_key {
                    let left = clone_link(&n.borrow().left);
                    n.borrow_mut().left = Self::insert_rec(left, key);
                } else if key > node_key {
                    let right = clone_link(&n.borrow().right);
                    n.borrow_mut().right = Self::insert_rec(right, key);
                } else {
                    return Some(n);
                }
                Some(rebalance(n))
            }
        }
    }

    pub fn delete(&mut self, key: u32) -> bool {
        let (new_root, deleted) = Self::delete_rec(clone_link(&self.root), key);
        self.root = new_root;
        deleted
    }

    fn delete_rec(node: AVLLink, key: u32) -> (AVLLink, bool) {
        match node {
            None => (None, false),
            Some(n) => {
                let node_key = n.borrow().key;
                if key < node_key {
                    let left = clone_link(&n.borrow().left);
                    let (new_left, deleted) = Self::delete_rec(left, key);
                    n.borrow_mut().left = new_left;
                    let new_root = rebalance(n);
                    (Some(new_root), deleted)
                } else if key > node_key {
                    let right = clone_link(&n.borrow().right);
                    let (new_right, deleted) = Self::delete_rec(right, key);
                    n.borrow_mut().right = new_right;
                    let new_root = rebalance(n);
                    (Some(new_root), deleted)
                } else {
                    let left = clone_link(&n.borrow().left);
                    let right = clone_link(&n.borrow().right);

                    match (left, right) {
                        (None, None) => (None, true),
                        (Some(l), None) => (Some(l), true),
                        (None, Some(r)) => (Some(r), true),
                        (Some(_), Some(r)) => {
                            let successor = Self::find_min(&r);
                            let successor_key = successor.borrow().key;
                            n.borrow_mut().key = successor_key;
                            let right2 = clone_link(&n.borrow().right);
                            let (new_right, _) = Self::delete_rec(right2, successor_key);
                            n.borrow_mut().right = new_right;
                            let new_root = rebalance(n);
                            (Some(new_root), true)
                        }
                    }
                }
            }
        }
    }

    fn find_min(node: &Rc<RefCell<AVLNode>>) -> Rc<RefCell<AVLNode>> {
        let left = clone_link(&node.borrow().left);
        match left {
            None => Rc::clone(node),
            Some(l) => Self::find_min(&l),
        }
    }

    pub fn count_leaves(&self) -> usize {
        Self::count_leaves_rec(&self.root)
    }

    fn count_leaves_rec(node: &AVLLink) -> usize {
        match node {
            None => 0,
            Some(n) => {
                let left = clone_link(&n.borrow().left);
                let right = clone_link(&n.borrow().right);
                if left.is_none() && right.is_none() {
                    1
                } else {
                    Self::count_leaves_rec(&left) + Self::count_leaves_rec(&right)
                }
            }
        }
    }

    pub fn height(&self) -> usize {
        node_height(&self.root).max(0) as usize
    }

    pub fn print_inorder(&self) {
        print!("In-order: [");
        Self::inorder_rec(&self.root);
        println!("]");
    }

    fn inorder_rec(node: &AVLLink) {
        if let Some(n) = node {
            let left = clone_link(&n.borrow().left);
            let right = clone_link(&n.borrow().right);
            Self::inorder_rec(&left);
            print!("{} ", n.borrow().key);
            Self::inorder_rec(&right);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn print_tree(&self) {
        if self.is_empty() {
            println!("(Empty AVL Tree)");
            return;
        }
        println!("┌─── AVL Tree ─────────────────────────");
        Self::print_tree_rec(&self.root, "", false);
        println!("└──────────────────────────────────────");
    }

    fn print_tree_rec(node: &AVLLink, prefix: &str, is_left: bool) {
        if let Some(n) = node {
            let right = clone_link(&n.borrow().right);
            let left = clone_link(&n.borrow().left);
            let key = n.borrow().key;
            let bf = balance_factor(&n);

            let child_prefix = if is_left {
                format!("{}│   ", prefix)
            } else {
                format!("{}    ", prefix)
            };
            Self::print_tree_rec(&right, &child_prefix, false);

            let connector = if is_left { "└── " } else { "┌── " };
            println!("{}{}{}(bf:{})", prefix, connector, key, bf);

            let child_prefix2 = if is_left {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            Self::print_tree_rec(&left, &child_prefix2, true);
        }
    }

    pub fn search(&self, key: u32) -> bool {
        Self::search_rec(&self.root, key)
    }

    fn search_rec(node: &AVLLink, key: u32) -> bool {
        match node {
            None => false,
            Some(n) => {
                let k = n.borrow().key;
                if key == k {
                    true
                } else if key < k {
                    let left = clone_link(&n.borrow().left);
                    Self::search_rec(&left, key)
                } else {
                    let right = clone_link(&n.borrow().right);
                    Self::search_rec(&right, key)
                }
            }
        }
    }
}

impl Default for AVLTree {
    fn default() -> Self {
        Self::new()
    }
}
