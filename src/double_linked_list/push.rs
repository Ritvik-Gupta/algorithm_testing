use super::{DoubleLinkedList, Node};

use super::result::{DeLLError, DeLLResult};
use std::rc::Rc;

impl<T> DoubleLinkedList<T> {
    pub fn push_front(&mut self, store: T) -> DeLLResult<()> {
        let new_head = Node::new(store);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
            }
            None => {
                self.tail = Some(new_head.clone());
            }
        }
        self.head = Some(new_head);
        self.length.increment();
        Ok(())
    }

    pub fn push_back(&mut self, store: T) -> DeLLResult<()> {
        let new_tail = Node::new(store);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
            }
        }
        self.tail = Some(new_tail);
        self.length.increment();
        Ok(())
    }

    pub fn push_middle(&mut self, insert_pos: u16, store: T) -> DeLLResult<()> {
        if insert_pos > *self.length {
            return Err(DeLLError::Index {
                max_possible_value: *self.length,
                provided_value: insert_pos,
            });
        }

        if insert_pos == 0 {
            return self.push_front(store);
        } else if insert_pos == *self.length {
            return self.push_back(store);
        }

        let mut this_node = self.head.as_ref().map(Rc::clone);
        let mut pos = 1;
        while let Some(prev_node) = this_node {
            if pos == insert_pos {
                let new_node = Node::new(store);
                {
                    let temp = prev_node.borrow();
                    let next_node = temp.next.as_ref().unwrap();

                    new_node.borrow_mut().next = Some(next_node.clone());
                    next_node.borrow_mut().prev = Some(new_node.clone());
                }
                new_node.borrow_mut().prev = Some(prev_node.clone());
                prev_node.borrow_mut().next = Some(new_node.clone());
                self.length.increment();
                return Ok(());
            }
            pos += 1;
            this_node = prev_node.borrow().next.as_ref().map(Rc::clone);
        }
        Err(DeLLError::Unexpected(None))
    }
}
