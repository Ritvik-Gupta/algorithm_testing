#[cfg(test)]
mod test;

mod format;
mod iter;
mod peek;
mod pop;
mod push;
mod result;

use std::cell::RefCell;
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

use services::unsigned_counter::UnsignedCounter;

pub struct DoubleLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: UnsignedCounter,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
            length: UnsignedCounter::at(0),
        }
    }

    pub fn size(&self) -> u16 {
        *self.length
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for DoubleLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_ok() {}
    }
}

pub struct IntoIter<T>(DoubleLinkedList<T>);

pub use peek::*;
pub use pop::*;
pub use push::*;
