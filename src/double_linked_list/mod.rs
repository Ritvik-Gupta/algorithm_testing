#[cfg(test)]
mod test;

mod format;
mod iter;
mod peek;
mod pop;
mod push;

use std::cell::RefCell;
use std::cmp::max;
use std::convert::TryInto;
use std::rc::Rc;

struct Node<T> {
    stored: T,
    next: Link<T>,
    prev: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            stored: elem,
            prev: None,
            next: None,
        }))
    }
}

struct SizeWrapper(u16);

impl SizeWrapper {
    fn increment(&mut self) {
        self.0 += 1;
    }
    fn decrement(&mut self) {
        self.0 = max(i32::from(self.0) - 1, 0).try_into().unwrap();
    }
}

impl std::ops::Deref for SizeWrapper {
    type Target = u16;

    fn deref(&self) -> &u16 {
        &self.0
    }
}

pub struct DoubleLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: SizeWrapper,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
            length: SizeWrapper(0),
        }
    }

    pub fn size(&self) -> u16 {
        *self.length
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(DoubleLinkedList<T>);

pub use format::*;
pub use iter::*;
pub use peek::*;
pub use pop::*;
pub use push::*;
