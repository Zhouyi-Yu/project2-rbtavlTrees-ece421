// =============================================================================
// rbt.rs — Red-Black Tree
// =============================================================================
//
// WHAT IS A RED-BLACK TREE?
// A Red-Black Tree (RBT) is a self-balancing binary search tree. Every node
// stores an extra bit of information: its COLOR (Red or Black). By enforcing
// five rules about these colors, the tree guarantees that no path from root to
// leaf is more than twice as long as any other, which keeps operations O(log n).
//
// THE FIVE RULES:
//   1. Every node is either Red or Black.
//   2. The root is always Black.
//   3. All "null" leaves are considered Black.
//   4. A Red node's children must both be Black (no two reds in a row).
//   5. Every path from a node to its descendant null-leaves has the same
//      number of Black nodes.
//
// HOW WE STORE THE TREE IN RUST:
// Rust's ownership system makes linked structures tricky. We use:
//   - Rc<RefCell<...>>  so multiple parts of the tree can hold references
//     to the same node (Rc = reference counted pointer, RefCell = allows
//     mutation even through shared references).
//   - Option<...>       to represent "this pointer might be null / empty".
//
// So a "link to a child" looks like:  Option<Rc<RefCell<Node>>>
// which reads: "optionally, a shared, mutable reference to a Node".
// =============================================================================

use std::cell::RefCell;
use std::fmt::{self, Display};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "R"),
            Color::Black => write!(f, "B"),
        }
    }
}

pub struct Node {
    pub key: u32,
    pub color: Color,
    pub left: RBLink,
    pub right: RBLink,
    pub parent: RBLink,
}

pub type RBLink = Option<Rc<RefCell<Node>>>;

impl Node {
    pub fn new(key: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            color: Color::Red,
            left: None,
            right: None,
            parent: None,
        }))
    }
}

pub struct RedBlackTree {
    root: RBLink,
}

fn color_of(node: &RBLink) -> Color {
    match node {
        None => Color::Black,
        Some(n) => n.borrow().color,
    }
}

fn clone_link(link: &RBLink) -> RBLink {
    link.as_ref().map(Rc::clone)
}

fn left_of(node: &RBLink) -> RBLink {
    node.as_ref()
        .map(|n| clone_link(&n.borrow().left))
        .flatten()
}

fn right_of(node: &RBLink) -> RBLink {
    node.as_ref()
        .map(|n| clone_link(&n.borrow().right))
        .flatten()
}

fn parent_of(node: &RBLink) -> RBLink {
    node.as_ref()
        .map(|n| clone_link(&n.borrow().parent))
        .flatten()
}

fn grandparent_of(node: &RBLink) -> RBLink {
    parent_of(&parent_of(node))
}

fn set_color(node: &RBLink, color: Color) {
    if let Some(n) = node {
        n.borrow_mut().color = color;
    }
}

fn same_node(a: &RBLink, b: &RBLink) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => Rc::ptr_eq(x, y),
        (None, None) => true,
        _ => false,
    }
}

impl RedBlackTree {
    fn rotate_left(&mut self, x_link: &RBLink) {
        let x = match x_link {
            Some(n) => Rc::clone(n),
            None => return,
        };

        let y = match clone_link(&x.borrow().right) {
            Some(n) => n,
            None => return,
        };

        let y_left = clone_link(&y.borrow().left);
        x.borrow_mut().right = clone_link(&y_left);
        if let Some(ref yl) = y_left {
            yl.borrow_mut().parent = Some(Rc::clone(&x));
        }

        let x_parent = clone_link(&x.borrow().parent);
        y.borrow_mut().parent = clone_link(&x_parent);

        match &x_parent {
            None => {
                self.root = Some(Rc::clone(&y));
            }
            Some(p) => {
                let p_left = clone_link(&p.borrow().left);
                if same_node(&p_left, &Some(Rc::clone(&x))) {
                    p.borrow_mut().left = Some(Rc::clone(&y));
                } else {
                    p.borrow_mut().right = Some(Rc::clone(&y));
                }
            }
        }

        y.borrow_mut().left = Some(Rc::clone(&x));
        x.borrow_mut().parent = Some(Rc::clone(&y));
    }

    fn rotate_right(&mut self, y_link: &RBLink) {
        let y = match y_link {
            Some(n) => Rc::clone(n),
            None => return,
        };

        let x = match clone_link(&y.borrow().left) {
            Some(n) => n,
            None => return,
        };

        let x_right = clone_link(&x.borrow().right);
        y.borrow_mut().left = clone_link(&x_right);
        if let Some(ref xr) = x_right {
            xr.borrow_mut().parent = Some(Rc::clone(&y));
        }

        let y_parent = clone_link(&y.borrow().parent);
        x.borrow_mut().parent = clone_link(&y_parent);

        match &y_parent {
            None => {
                self.root = Some(Rc::clone(&x));
            }
            Some(p) => {
                let p_right = clone_link(&p.borrow().right);
                if same_node(&p_right, &Some(Rc::clone(&y))) {
                    p.borrow_mut().right = Some(Rc::clone(&x));
                } else {
                    p.borrow_mut().left = Some(Rc::clone(&x));
                }
            }
        }

        x.borrow_mut().right = Some(Rc::clone(&y));
        y.borrow_mut().parent = Some(Rc::clone(&x));
    }

    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn insert(&mut self, key: u32) {
        let z = Node::new(key);

        let mut parent: RBLink = None;
        let mut current = clone_link(&self.root);

        while let Some(ref cur_node) = clone_link(&current) {
            parent = clone_link(&current);
            let cur_key = cur_node.borrow().key;
            if key < cur_key {
                current = left_of(&current);
            } else if key > cur_key {
                current = right_of(&current);
            } else {
                return;
            }
        }

        z.borrow_mut().parent = clone_link(&parent);
        match &parent {
            None => {
                self.root = Some(Rc::clone(&z));
            }
            Some(p) => {
                if key < p.borrow().key {
                    p.borrow_mut().left = Some(Rc::clone(&z));
                } else {
                    p.borrow_mut().right = Some(Rc::clone(&z));
                }
            }
        }

        self.insert_fixup(Some(Rc::clone(&z)));

        set_color(&self.root, Color::Black);
    }

    fn insert_fixup(&mut self, mut z: RBLink) {
        while color_of(&parent_of(&z)) == Color::Red {
            let parent = parent_of(&z);
            let grandparent = grandparent_of(&z);

            if same_node(&parent, &left_of(&grandparent)) {
                let uncle = right_of(&grandparent);

                if color_of(&uncle) == Color::Red {
                    set_color(&parent, Color::Black);
                    set_color(&uncle, Color::Black);
                    set_color(&grandparent, Color::Red);
                    z = grandparent;
                } else {
                    if same_node(&z, &right_of(&parent)) {
                        z = parent;
                        self.rotate_left(&z);
                    }
                    let p = parent_of(&z);
                    let gp = grandparent_of(&z);
                    set_color(&p, Color::Black);
                    set_color(&gp, Color::Red);
                    self.rotate_right(&gp);
                }
            } else {
                let uncle = left_of(&grandparent);

                if color_of(&uncle) == Color::Red {
                    set_color(&parent, Color::Black);
                    set_color(&uncle, Color::Black);
                    set_color(&grandparent, Color::Red);
                    z = grandparent;
                } else {
                    if same_node(&z, &left_of(&parent)) {
                        z = parent;
                        self.rotate_right(&z);
                    }
                    let p = parent_of(&z);
                    let gp = grandparent_of(&z);
                    set_color(&p, Color::Black);
                    set_color(&gp, Color::Red);
                    self.rotate_left(&gp);
                }
            }
        }
        set_color(&self.root, Color::Black);
    }

    pub fn delete(&mut self, key: u32) -> bool {
        let z = self.find_node(key);
        let z = match z {
            None => return false,
            Some(n) => n,
        };

        self.delete_node(z);
        true
    }

    fn find_node(&self, key: u32) -> RBLink {
        let mut current = clone_link(&self.root);
        while let Some(ref node) = clone_link(&current) {
            let k = node.borrow().key;
            if key == k {
                return current;
            } else if key < k {
                current = left_of(&current);
            } else {
                current = right_of(&current);
            }
        }
        None
    }

    fn delete_node(&mut self, z: Rc<RefCell<Node>>) {
        let y;
        let original_y_color;

        let z_left = clone_link(&z.borrow().left);
        let z_right = clone_link(&z.borrow().right);

        if z_left.is_none() || z_right.is_none() {
            y = Rc::clone(&z);
        } else {
            y = self.minimum(z_right.as_ref().unwrap());
        }

        original_y_color = y.borrow().color;

        let x: RBLink = {
            let y_left = clone_link(&y.borrow().left);
            let y_right = clone_link(&y.borrow().right);
            if y_left.is_some() { y_left } else { y_right }
        };

        let x_parent: RBLink;

        let y_parent = clone_link(&y.borrow().parent);

        if let Some(ref xc) = x {
            xc.borrow_mut().parent = clone_link(&y_parent);
        }
        x_parent = clone_link(&y_parent);

        match &y_parent {
            None => {
                self.root = clone_link(&x);
            }
            Some(p) => {
                let p_left = clone_link(&p.borrow().left);
                if same_node(&p_left, &Some(Rc::clone(&y))) {
                    p.borrow_mut().left = clone_link(&x);
                } else {
                    p.borrow_mut().right = clone_link(&x);
                }
            }
        }

        if !Rc::ptr_eq(&y, &z) {
            z.borrow_mut().key = y.borrow().key;
        }

        if original_y_color == Color::Black {
            self.delete_fixup(x, x_parent);
        }
    }

    fn minimum(&self, node: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let mut current = Rc::clone(node);
        loop {
            let left = clone_link(&current.borrow().left);
            match left {
                None => break,
                Some(l) => current = l,
            }
        }
        current
    }

    fn delete_fixup(&mut self, mut x: RBLink, mut x_parent: RBLink) {
        while !same_node(&x, &self.root) && color_of(&x) == Color::Black {
            let xp = clone_link(&x_parent);
            let xp_left = left_of(&xp);

            if same_node(&x, &xp_left) {
                let mut w = right_of(&xp);

                if color_of(&w) == Color::Red {
                    set_color(&w, Color::Black);
                    set_color(&xp, Color::Red);
                    self.rotate_left(&xp);
                    w = right_of(&x_parent);
                }

                if color_of(&left_of(&w)) == Color::Black && color_of(&right_of(&w)) == Color::Black
                {
                    set_color(&w, Color::Red);
                    x = clone_link(&x_parent);
                    x_parent = parent_of(&x);
                } else {
                    if color_of(&right_of(&w)) == Color::Black {
                        set_color(&left_of(&w), Color::Black);
                        set_color(&w, Color::Red);
                        self.rotate_right(&w);
                        w = right_of(&x_parent);
                    }
                    set_color(&w, color_of(&x_parent));
                    set_color(&x_parent, Color::Black);
                    set_color(&right_of(&w), Color::Black);
                    self.rotate_left(&x_parent);
                    x = clone_link(&self.root);
                    x_parent = None;
                }
            } else {
                let mut w = left_of(&xp);

                if color_of(&w) == Color::Red {
                    set_color(&w, Color::Black);
                    set_color(&xp, Color::Red);
                    self.rotate_right(&xp);
                    w = left_of(&x_parent);
                }

                if color_of(&right_of(&w)) == Color::Black && color_of(&left_of(&w)) == Color::Black
                {
                    set_color(&w, Color::Red);
                    x = clone_link(&x_parent);
                    x_parent = parent_of(&x);
                } else {
                    if color_of(&left_of(&w)) == Color::Black {
                        set_color(&right_of(&w), Color::Black);
                        set_color(&w, Color::Red);
                        self.rotate_left(&w);
                        w = left_of(&x_parent);
                    }
                    set_color(&w, color_of(&x_parent));
                    set_color(&x_parent, Color::Black);
                    set_color(&left_of(&w), Color::Black);
                    self.rotate_right(&x_parent);
                    x = clone_link(&self.root);
                    x_parent = None;
                }
            }
        }
        set_color(&x, Color::Black);
    }

    pub fn count_leaves(&self) -> usize {
        Self::count_leaves_rec(&self.root)
    }

    fn count_leaves_rec(node: &RBLink) -> usize {
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
        Self::height_rec(&self.root)
    }

    fn height_rec(node: &RBLink) -> usize {
        match node {
            None => 0,
            Some(n) => {
                let left = clone_link(&n.borrow().left);
                let right = clone_link(&n.borrow().right);
                1 + Self::height_rec(&left).max(Self::height_rec(&right))
            }
        }
    }

    pub fn print_inorder(&self) {
        print!("In-order: [");
        Self::inorder_rec(&self.root);
        println!("]");
    }

    fn inorder_rec(node: &RBLink) {
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
            println!("(Empty Red-Black Tree)");
            return;
        }
        println!("┌─── Red-Black Tree ───────────────────");
        Self::print_tree_rec(&self.root, "", false);
        println!("└──────────────────────────────────────");
    }

    fn print_tree_rec(node: &RBLink, prefix: &str, is_left: bool) {
        if let Some(n) = node {
            let right = clone_link(&n.borrow().right);
            let left = clone_link(&n.borrow().left);
            let key = n.borrow().key;
            let color = n.borrow().color;

            let child_prefix = if is_left {
                format!("{}│   ", prefix)
            } else {
                format!("{}    ", prefix)
            };
            Self::print_tree_rec(&right, &child_prefix, false);

            let connector = if is_left { "└-- " } else { "┌-- " };
            let color_tag = match color {
                Color::Red => "\x1b[31m(R)\x1b[0m",
                Color::Black => "(B)",
            };
            println!("{}{}{}{}", prefix, connector, key, color_tag);

            let child_prefix2 = if is_left {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            Self::print_tree_rec(&left, &child_prefix2, true);
        }
    }

    pub fn search(&self, key: u32) -> bool {
        self.find_node(key).is_some()
    }
}

impl Default for RedBlackTree {
    fn default() -> Self {
        Self::new()
    }
}
