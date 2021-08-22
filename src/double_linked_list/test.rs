use super::DoubleLinkedList;

use super::result::{DeLLError, DeLLResult};

#[test]
fn basics() -> DeLLResult<()> {
    let mut list = DoubleLinkedList::new();

    // Check empty list behaves right
    assert_eq!(list.pop_front(), Err(DeLLError::Empty));

    // Populate list
    list.push_front(1)?;
    list.push_front(2)?;
    list.push_front(3)?;

    // Check normal removal
    assert_eq!(list.pop_front()?, 3);
    assert_eq!(list.pop_front()?, 2);

    // Push some more just to make sure nothing's corrupted
    list.push_front(4)?;
    list.push_front(5)?;

    // Check normal removal
    assert_eq!(list.pop_front()?, 5);
    assert_eq!(list.pop_front()?, 4);

    // Check exhaustion
    assert_eq!(list.pop_front()?, 1);
    assert_eq!(list.pop_front(), Err(DeLLError::Empty));

    // ---- back -----

    // Check empty list behaves right
    assert_eq!(list.pop_back(), Err(DeLLError::Empty));

    // Populate list
    list.push_back(1)?;
    list.push_back(2)?;
    list.push_back(3)?;

    // Check normal removal
    assert_eq!(list.pop_back()?, 3);
    assert_eq!(list.pop_back()?, 2);

    // Push some more just to make sure nothing's corrupted
    list.push_back(4)?;
    list.push_back(5)?;

    // Check normal removal
    assert_eq!(list.pop_back()?, 5);
    assert_eq!(list.pop_back()?, 4);

    // Check exhaustion
    assert_eq!(list.pop_back()?, 1);
    assert_eq!(list.pop_back(), Err(DeLLError::Empty));

    Ok(())
}

#[test]
fn peek() -> DeLLResult<()> {
    let mut list = DoubleLinkedList::new();
    assert!(list.peek_front().is_none());
    assert!(list.peek_back().is_none());
    assert!(list.peek_front_mut().is_none());
    assert!(list.peek_back_mut().is_none());

    list.push_front(1)?;
    list.push_front(2)?;
    list.push_front(3)?;

    assert_eq!(&*list.peek_front().unwrap(), &3);
    assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
    assert_eq!(&*list.peek_back().unwrap(), &1);
    assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);

    Ok(())
}

#[test]
fn into_iter() -> DeLLResult<()> {
    let mut list = DoubleLinkedList::new();
    list.push_front(1)?;
    list.push_front(2)?;
    list.push_front(3)?;

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    Ok(())
}

#[test]
fn middle_operations() -> DeLLResult<()> {
    let mut list = DoubleLinkedList::new();

    assert_eq!(list.pop_front(), Err(DeLLError::Empty));

    list.push_front(1)?;
    list.push_front(2)?;
    list.push_front(3)?;

    list.traverse_head();
    list.traverse_tail();

    list.push_middle(2, 10)?;

    list.traverse_head();
    list.traverse_tail();

    assert_eq!(list.pop_middle(1)?, 2);

    list.traverse_head();
    list.traverse_tail();

    assert_eq!(list.pop_back()?, 1);
    assert_eq!(list.pop_back()?, 10);
    assert_eq!(list.pop_back()?, 3);

    Ok(())
}
