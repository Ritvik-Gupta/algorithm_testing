use super::DoubleLinkedList;

use std::rc::Rc;

impl<T> DoubleLinkedList<T> {
    pub fn pop_front(&mut self) -> Option<T> {
        self.length.decrement();
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().stored
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.length.decrement();
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().stored
        })
    }

    pub fn pop_middle(&mut self, delete_pos: u16) -> Option<T> {
        if delete_pos >= *self.length {
            panic!("Index out of bounds");
        }

        if delete_pos == 0 {
            return self.pop_front();
        } else if delete_pos == *self.length - 1 {
            return self.pop_back();
        }

        let mut this_node: *mut _ = &mut self.head;
        let mut pos = 0;
        unsafe {
            while let Some(node) = &*this_node {
                if pos == delete_pos {
                    let temp = node.borrow();
                    let prev_node = temp.prev.as_ref().unwrap();
                    let next_node = temp.next.as_ref().unwrap();

                    node.borrow_mut().prev = None;
                    node.borrow_mut().next = None;
                    prev_node.borrow_mut().next = Some(next_node.clone());
                    next_node.borrow_mut().prev = Some(prev_node.clone());

                    self.length.decrement();
                    return Some(
                        Rc::try_unwrap((*this_node).take().unwrap())
                            .ok()
                            .unwrap()
                            .into_inner()
                            .stored,
                    );
                }
                pos += 1;
                this_node = &mut node.borrow_mut().next;
            }
        }
        panic!("Unexpected Error occurred");
    }
}
