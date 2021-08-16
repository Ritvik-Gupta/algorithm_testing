use super::{
   binary_child::BinaryChild::{LEFT, RIGHT},
   BinaryTree,
};
use std::collections::LinkedList;
use std::fmt::{Debug, Display, Formatter, Result};
use IterOrder::{INFIX, POSTFIX, PREFIX};
use TreeCollector::{Emit, TreeNode};

pub enum TreeCollector<'a, T> {
   TreeNode(&'a BinaryTree<T>),
   Emit(&'a T),
}

impl<'a, T: Display> Debug for TreeCollector<'a, T> {
   fn fmt(&self, formatter: &mut Formatter) -> Result {
      match self {
         TreeNode(binary_tree) => write!(formatter, "( {} )", binary_tree),
         Emit(stored) => write!(formatter, "{}", stored),
      }
   }
}

pub enum IterOrder {
   PREFIX = 0,
   INFIX = 1,
   POSTFIX = 2,
}

pub struct Iter<'a, T> {
   stack: LinkedList<TreeCollector<'a, T>>,
   order: IterOrder,
}

impl<'a, T> Iter<'a, T> {
   pub fn new(binary_tree: &'a BinaryTree<T>, order: IterOrder) -> Iter<'a, T> {
      let mut stack = LinkedList::new();
      stack.push_front(TreeNode(binary_tree));
      Iter { stack, order }
   }
}

impl<'a, T: Display> Iterator for Iter<'a, T> {
   type Item = &'a T;

   fn next(&mut self) -> Option<Self::Item> {
      while let Some(tree_collector) = self.stack.pop_front() {
         match tree_collector {
            TreeNode(binary_tree) => {
               vec![
                  if let PREFIX = self.order {
                     Some(Emit(&binary_tree.store))
                  } else {
                     None
                  },
                  if let Some(child) = binary_tree.get(LEFT) {
                     Some(TreeNode(child))
                  } else {
                     None
                  },
                  if let INFIX = self.order {
                     Some(Emit(&binary_tree.store))
                  } else {
                     None
                  },
                  if let Some(child) = binary_tree.get(RIGHT) {
                     Some(TreeNode(child))
                  } else {
                     None
                  },
                  if let POSTFIX = self.order {
                     Some(Emit(&binary_tree.store))
                  } else {
                     None
                  },
               ]
               .into_iter()
               .filter_map(|elm| elm)
               .rev()
               .for_each(|elm| self.stack.push_front(elm));
            }
            Emit(store) => return Some(store),
         }
      }
      None
   }
}
