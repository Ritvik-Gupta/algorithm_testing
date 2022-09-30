use super::{DoubleLinkedList, Node};

use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

struct LinkPrinter<'a, T>(&'a Rc<RefCell<Node<T>>>);

impl<'a, T: std::fmt::Debug> std::fmt::Debug for LinkPrinter<'a, T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(next_node1) = &self.0.borrow().prev {
            write!(formatter, "{:p}", next_node1)?;
        } else {
            write!(formatter, "NULL")?;
        }
        write!(formatter, " <- {:p} -> ", self.0)?;
        if let Some(next_node) = &self.0.borrow().next {
            write!(formatter, "{:p}", next_node)?;
        } else {
            write!(formatter, "NULL")?;
        }

        Ok(())
    }
}

impl<T: std::fmt::Debug> DoubleLinkedList<T> {
    pub fn traverse_head(&self) {
        let mut this_node: *const _ = &self.head;

        println!("[");
        unsafe {
            while !this_node.is_null() {
                this_node = match &*this_node {
                    Some(node) => {
                        println!("\t{:?}", LinkPrinter(node));
                        &node.borrow().next
                    }
                    None => ptr::null(),
                }
            }
        }
        println!("]\n");
    }

    pub fn traverse_tail(&self) {
        let mut this_node: *const _ = &self.tail;

        println!("[");
        unsafe {
            while !this_node.is_null() {
                this_node = match &*this_node {
                    Some(node) => {
                        println!("\t{:?}", LinkPrinter(node));
                        &node.borrow().prev
                    }
                    None => ptr::null(),
                }
            }
        }
        println!("]\n");
    }
}
