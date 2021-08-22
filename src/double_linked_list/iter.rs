use super::{DoubleLinkedList, IntoIter};

impl<T> Drop for DoubleLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_ok() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front().ok()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back().ok()
    }
}
