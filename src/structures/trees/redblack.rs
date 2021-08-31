use crate::structures::{BinaryTreePrint, BinaryTreeUtil, BinaryTreeValidator};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::rc::{Rc, Weak};

/// A (bad) implementaion of a RedBlack Tree.
///
/// CTCI does not require an RB tree, but I wanted to implement one to tackle a "harder" data
/// structure and play around with Rust a little more. A lot of choices here were driven by
/// curiosity about Rust and some of its features, so the API is strange to say the least.
/// In addition trying to consolidate some of the logic inside of the Tree `BinaryTreeUtil` +
/// `BinaryTreePrint` + `BinaryTreeValidator` was also purely as a way of getting a little more
/// experience with Traits and how they work.
///
/// NOTE: I will most likely not update this code other than adding a `delete` method, as well as a
/// couple additional helpers in the `crate::structures::utils` module.
#[derive(Debug)]
pub struct RBTree<T> {
    pub root: Child<T>,
}

type BareChild<T> = Rc<RefCell<RBNode<T>>>;
pub type Child<T> = Option<BareChild<T>>;
type BareParent<T> = Weak<RefCell<RBNode<T>>>;
pub type Parent<T> = Option<BareParent<T>>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    Red,
}

#[derive(Debug)]
pub struct RBNode<T> {
    pub val: T,
    pub parent: Parent<T>,
    pub left: Child<T>,
    pub right: Child<T>,
    count: u32,
    color: Color,
}

#[derive(Debug)]
struct Family<T> {
    node: BareChild<T>,
    parent: BareChild<T>,
    uncle: Child<T>,
    grandparent: BareChild<T>,
}

impl<T> Family<T> {
    fn new(
        node: BareChild<T>,
        parent: BareChild<T>,
        uncle: Child<T>,
        grandparent: BareChild<T>,
    ) -> Self {
        Self {
            node,
            parent,
            uncle,
            grandparent,
        }
    }
}

#[derive(Debug)]
enum BalanceState<T> {
    Left(Branch<T>),
    Right(Branch<T>),
    ReColor(Family<T>),
    Root(BareChild<T>),
    Done,
}

#[derive(Debug)]
enum Branch<T> {
    Left(Family<T>),
    Right(Family<T>),
}

pub trait Colored {
    fn color(&self) -> Color;
}

impl<T> Colored for Child<T> {
    fn color(&self) -> Color {
        if let Some(node) = self {
            return node.borrow().color;
        }
        Color::Black
    }
}

impl<T> Colored for BareChild<T> {
    fn color(&self) -> Color {
        self.borrow().color
    }
}

pub trait NodeActions<T>
where
    Self: Sized,
    Self: BinaryTreeUtil + Colored,
{
    fn new_node(val: T) -> Self;
    fn new_child(&self, val: T) -> Option<Self>;
    fn parent(&self) -> Option<Self>;
    fn uncle(&self) -> Option<Self>;
    fn grandparent(&self) -> Option<Self>;
    fn add_left(&self, val: T) -> Option<Self>;
    fn add_right(&self, val: T) -> Option<Self>;
    fn set_left(&self, child: Option<Self>);
    fn set_right(&self, child: Option<Self>);
    fn set_color(&self, color: Color);
    fn eq(&self, node: Self) -> bool;
    fn is_right(&self, node: Self) -> bool;
    fn is_left(&self, node: Self) -> bool;
    fn is_parent(&self, node: Self) -> bool;
    fn swap_colors(&self, node: Self);
    fn increment_count(&self);
}

impl<T> NodeActions<T> for BareChild<T> {
    fn new_node(val: T) -> Self {
        Rc::new(RefCell::new(RBNode {
            val,
            parent: None,
            left: None,
            right: None,
            color: Color::Red,
            count: 1,
        }))
    }

    fn new_child(&self, val: T) -> Option<Self> {
        let out = Self::new_node(val);
        out.borrow_mut().parent = Some(Rc::downgrade(&self));
        Some(out)
    }

    fn parent(&self) -> Option<Self> {
        let parent = self.borrow().parent.as_ref().map(|n| n.clone());
        if let Some(parent) = parent {
            return parent.upgrade();
        }
        None
    }

    fn uncle(&self) -> Option<Self> {
        if let Some(grandparent) = self.grandparent() {
            let parent = self
                .parent()
                .expect("parent can't be None if grandparent exists");
            if grandparent.is_left(parent) {
                return grandparent.right();
            }
            return grandparent.left();
        }
        None
    }

    fn grandparent(&self) -> Option<Self> {
        let parent = self.parent();
        if let Some(parent) = parent {
            return parent.parent();
        }
        None
    }

    fn add_left(&self, val: T) -> Option<Self> {
        let child = self.new_child(val);
        self.borrow_mut().left = child;
        self.left()
    }

    fn add_right(&self, val: T) -> Option<Self> {
        let child = self.new_child(val);
        self.borrow_mut().right = child;
        self.right()
    }

    fn set_left(&self, mut child: Option<Self>) {
        if let Some(child) = &child {
            child.borrow_mut().parent = Some(Rc::downgrade(&self));
        }
        self.borrow_mut().left = child.take();
    }

    fn set_right(&self, mut child: Option<Self>) {
        if let Some(child) = &child {
            child.borrow_mut().parent = Some(Rc::downgrade(&self));
        }
        self.borrow_mut().right = child.take();
    }

    fn set_color(&self, color: Color) {
        self.borrow_mut().color = color
    }

    fn eq(&self, node: Self) -> bool {
        Rc::ptr_eq(&self, &node)
    }

    fn is_right(&self, node: Self) -> bool {
        if let Some(right) = self.right() {
            return Rc::ptr_eq(&right, &node);
        }
        false
    }

    fn is_left(&self, node: Self) -> bool {
        if let Some(left) = self.left() {
            return Rc::ptr_eq(&left, &node);
        }
        false
    }

    fn is_parent(&self, node: Self) -> bool {
        if let Some(parent) = self.parent() {
            return Rc::ptr_eq(&parent, &node);
        }
        false
    }

    fn swap_colors(&self, node: Self) {
        let color = node.color();
        node.set_color(self.color());
        self.set_color(color);
    }

    fn increment_count(&self) {
        self.borrow_mut().count += 1;
    }
}

impl<T> RBTree<T>
where
    T: Eq + PartialOrd + Ord + Debug + Display + Copy,
{
    pub fn from<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut result = Self::new();
        for item in iter {
            result.insert(item);
        }
        result
    }

    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert_node(&mut self, val: T) -> Child<T> {
        let mut cursor = self.root.as_ref().map(|n| n.clone());
        while let Some(node) = cursor {
            match val.cmp(&node.val()) {
                Ordering::Less => {
                    if node.left().is_none() {
                        return node.add_left(val);
                    }
                    cursor = node.left();
                }
                Ordering::Greater => {
                    if node.right().is_none() {
                        return node.add_right(val);
                    }
                    cursor = node.right();
                }
                Ordering::Equal => {
                    node.increment_count();
                    return Some(node.clone());
                }
            }
        }
        self.root = Some(BareChild::new_node(val));
        self.root.as_ref().map(|n| n.clone())
    }

    pub fn insert(&mut self, val: T) {
        let node = self.insert_node(val);
        let state = self.balance_state(node.unwrap());
        self.balance(state);
    }

    fn balance_state(&self, node: BareChild<T>) -> BalanceState<T> {
        let root = self
            .root
            .as_ref()
            .map(|n| n.clone())
            .expect("balance state should not be called if root is None");

        if node.eq(root) {
            return BalanceState::Root(node);
        }
        let parent = node.parent();
        if parent.color() == Color::Black {
            return BalanceState::Done;
        }
        // Parent is red so grandparent must exists;
        let parent = parent.expect("parent is red so can't be None");
        let grandparent = node
            .grandparent()
            .expect("expect parent is red so grandparent can't be None");
        let uncle = node.uncle();

        if uncle.color() == Color::Red {
            return BalanceState::ReColor(Family::new(node, parent, uncle, grandparent));
        }

        let inner_branch = if parent.is_left(Rc::clone(&node)) {
            Branch::Left(Family::new(
                node,
                parent.clone(),
                uncle,
                grandparent.clone(),
            ))
        } else {
            Branch::Right(Family::new(
                node,
                parent.clone(),
                uncle,
                grandparent.clone(),
            ))
        };

        if grandparent.is_left(parent) {
            return BalanceState::Left(inner_branch);
        }
        BalanceState::Right(inner_branch)
    }

    fn rotate_right(&mut self, g: BareChild<T>) {
        if let Some(left) = g.left() {
            left.borrow_mut().parent = g.parent().map(|n| Rc::downgrade(&n));
            left.swap(&g);
            // Naming is confusing since we swapped the pointers
            g.set_left(g.left());
            left.set_right(left.right());

            left.set_left(g.right());
            g.set_right(Some(left.clone()));
        }
    }

    fn rotate_left(&mut self, g: BareChild<T>) {
        if let Some(right) = g.right() {
            right.borrow_mut().parent = g.parent().map(|n| Rc::downgrade(&n));
            right.swap(&g);
            // Naming is confusing since we swapped the pointers
            g.set_right(g.right());
            right.set_left(right.left());

            right.set_right(g.left());
            g.set_left(Some(right.clone()));
        }
    }

    fn balance(&mut self, mut state: BalanceState<T>) {
        // State Machine :)
        loop {
            match state {
                BalanceState::Left(Branch::Left(family)) => {
                    self.rotate_right(family.grandparent.clone());
                    family.parent.swap_colors(family.grandparent);
                    state = BalanceState::Done;
                }
                BalanceState::Right(Branch::Right(family)) => {
                    self.rotate_left(family.grandparent.clone());
                    family.parent.swap_colors(family.grandparent);
                    state = BalanceState::Done;
                }
                BalanceState::Left(Branch::Right(family)) => {
                    self.rotate_left(family.parent.clone());
                    state = BalanceState::Left(Branch::Left(family));
                }
                BalanceState::Right(Branch::Left(family)) => {
                    self.rotate_right(family.parent.clone());
                    state = BalanceState::Right(Branch::Right(family));
                }
                BalanceState::ReColor(family) => {
                    family.parent.set_color(Color::Black);
                    family.grandparent.set_color(Color::Red);
                    if let Some(uncle) = family.uncle {
                        uncle.set_color(Color::Black);
                    }
                    state = self.balance_state(family.grandparent);
                }
                BalanceState::Root(root) => {
                    root.set_color(Color::Black);
                    state = BalanceState::Done
                }
                BalanceState::Done => break,
            }
        }
    }

    pub fn depth(&self) -> usize {
        if let Some(root) = &self.root {
            return root.depth();
        }
        0
    }

    pub fn is_valid_bst(&self) -> bool
    where
        T: PartialOrd + Copy,
    {
        if let Some(root) = &self.root {
            return root.is_valid_bst();
        }
        true
    }

    pub fn print(&self)
    where
        T: Display,
    {
        if let Some(root) = &self.root {
            return root.pprint();
        }
        println!("Empty Tree");
    }

    pub fn nodes(&self) -> Vec<BareChild<T>> {
        let mut nodes = Vec::new();
        let mut push = |node: &BareChild<T>| nodes.push(node.clone());
        if let Some(root) = &self.root {
            root.inorder(&mut push);
        }
        nodes
    }
}

/* Tree Utils */

impl<T> BinaryTreeUtil for BareChild<T> {
    fn left(&self) -> Option<Self> {
        self.borrow().left.as_ref().map(|n| n.clone())
    }

    fn right(&self) -> Option<Self> {
        self.borrow().right.as_ref().map(|n| n.clone())
    }
}

impl<T> BinaryTreePrint<T> for BareChild<T>
where
    T: Display,
{
    fn print_node(&self) -> String {
        format!("{} x {}", self.borrow().val, self.borrow().count)
    }
}

impl<T> BinaryTreeValidator<T> for BareChild<T>
where
    T: PartialOrd + Copy,
{
    fn val(&self) -> T {
        self.borrow().val
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::*;

    fn generate_random_bst() -> RBTree<i32> {
        let mut tree = RBTree::new();
        let mut rng = rand::thread_rng();
        for _ in 1..=100 {
            let random = rng.gen_range(0..200);
            tree.insert(random);
        }
        tree
    }

    #[test]
    fn rb_basic() {
        let tree = generate_random_bst();
        assert!(tree.is_valid_bst());
    }

    #[test]
    fn rb_inorder() {
        let tree = generate_random_bst();
        let mut out = vec![];
        let mut test = |node: &BareChild<i32>| {
            out.push(node.val());
        };
        if let Some(root) = tree.root {
            root.inorder(&mut test);
        }
        let mut result = out.clone();
        result.sort();
        assert_eq!(out, result);
    }
}
