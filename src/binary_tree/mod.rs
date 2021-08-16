pub mod binary_child;
pub mod iter;

use binary_child::BinaryChild::{self, LEFT, RIGHT};

pub struct BinaryTree<T> {
   store: T,
   left_child: Option<Box<BinaryTree<T>>>,
   right_child: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T> {
   pub fn new(root_store: T) -> BinaryTree<T> {
      BinaryTree {
         store: root_store,
         left_child: None,
         right_child: None,
      }
   }

   fn fetch_ref(&self, binary_child: BinaryChild) -> &Option<Box<BinaryTree<T>>> {
      match binary_child {
         LEFT => &self.left_child,
         RIGHT => &self.right_child,
      }
   }

   fn fetch_mut_ref(&mut self, binary_child: BinaryChild) -> &mut Option<Box<BinaryTree<T>>> {
      match binary_child {
         LEFT => &mut self.left_child,
         RIGHT => &mut self.right_child,
      }
   }

   pub fn has(&self, binary_child: BinaryChild) -> bool {
      self.fetch_ref(binary_child).is_some()
   }

   pub fn get(&self, binary_child: BinaryChild) -> Option<&Box<BinaryTree<T>>> {
      self.fetch_ref(binary_child).as_ref()
   }

   pub fn get_mut(&mut self, binary_child: BinaryChild) -> Option<&mut Box<BinaryTree<T>>> {
      self.fetch_mut_ref(binary_child).as_mut()
   }

   pub fn set(&mut self, binary_child: BinaryChild, store: T) {
      *self.fetch_mut_ref(binary_child) = Some(Box::new(BinaryTree::new(store)));
   }

   pub fn inorder(&self) -> Vec<&T> {
      let mut list = Vec::new();
      self
         .get(LEFT)
         .map(|child| list.append(&mut child.inorder()));
      list.push(&self.store);
      self
         .get(RIGHT)
         .map(|child| list.append(&mut child.inorder()));
      list
   }

   pub fn preorder(&self) -> Vec<&T> {
      let mut list = vec![&self.store];
      self
         .get(LEFT)
         .map(|child| list.append(&mut child.preorder()));
      self
         .get(RIGHT)
         .map(|child| list.append(&mut child.preorder()));
      list
   }

   pub fn postorder(&self) -> Vec<&T> {
      let mut list = Vec::new();
      self
         .get(LEFT)
         .map(|child| list.append(&mut child.postorder()));
      self
         .get(RIGHT)
         .map(|child| list.append(&mut child.postorder()));
      list.push(&self.store);
      list
   }
}

use iter::{Iter, IterOrder};

impl<T> BinaryTree<T> {
   pub fn iter(&self, iter_order: IterOrder) -> Iter<T> {
      Iter::new(self, iter_order)
   }
}

use std::fmt::{Display, Formatter, Result};

impl<T: Display> Display for BinaryTree<T> {
   fn fmt(&self, formatter: &mut Formatter) -> Result {
      write!(
         formatter,
         "{} {} {}",
         if self.has(LEFT) { "<<" } else { ".." },
         self.store,
         if self.has(RIGHT) { ">>" } else { ".." }
      )
   }
}

use std::ops::{Index, IndexMut, ShlAssign, ShrAssign};

impl<T> Index<BinaryChild> for BinaryTree<T> {
   type Output = BinaryTree<T>;

   fn index(&self, binary_child: BinaryChild) -> &Self::Output {
      self
         .fetch_ref(binary_child)
         .as_ref()
         .expect(&format!("{} child does not exist", binary_child.name()))
   }
}

impl<T> IndexMut<BinaryChild> for BinaryTree<T> {
   fn index_mut(&mut self, binary_child: BinaryChild) -> &mut Self::Output {
      self
         .fetch_mut_ref(binary_child)
         .as_mut()
         .expect(&format!("{} child does not exist", binary_child.name()))
   }
}

impl<T> ShlAssign<T> for BinaryTree<T> {
   fn shl_assign(&mut self, rhs: T) {
      self.set(LEFT, rhs);
   }
}

impl<T> ShrAssign<T> for BinaryTree<T> {
   fn shr_assign(&mut self, rhs: T) {
      self.set(RIGHT, rhs);
   }
}
