extern crate algorithms;

fn main() {
    let mut root = TreeNode::new(1);

    let parent_pointer = NonNull::from(&*root);

    match unsafe { &mut *root.as_mut().get_unchecked_mut() } {
        Node { left, right, .. } => {
            *left = TreeNode::new(2);
            match unsafe { &mut *left.as_mut().get_unchecked_mut() } {
                Node { parent, .. } => *parent = parent_pointer,
                END => panic!(),
            }
            *right = TreeNode::new(3);
            match unsafe { &mut *right.as_mut().get_unchecked_mut() } {
                Node { parent, .. } => *parent = parent_pointer,
                END => panic!(),
            }
        }
        END => {}
    }

    println!("{:#?}", unsafe { parent_pointer.as_ref() });
}

use std::{marker::PhantomPinned, pin::Pin, ptr::NonNull};

#[derive(Debug)]
enum TreeNode {
    Node {
        store: i32,
        parent: NonNull<TreeNode>,
        left: Pin<Box<TreeNode>>,
        right: Pin<Box<TreeNode>>,
        _pinned: PhantomPinned,
    },
    END,
}

use TreeNode::{Node, END};

impl TreeNode {
    fn new(store: i32) -> Pin<Box<Self>> {
        Box::pin(Node {
            store,
            parent: NonNull::dangling(),
            left: Box::pin(END),
            right: Box::pin(END),
            _pinned: PhantomPinned,
        })
    }
}

/*
macro_rules! chain {
    [
        $(
            $call_to_method: ident
            (
                $($args: expr),*
                $(,)?
            )
        )+
        -> $on_object: ident
    ] => [(
        $(
            $on_object.$call_to_method($($args)*)
        ),+
    )];
}
*/

/*
macro_rules! count_exprs {
    () => (0);
    ($head: expr) => (1);
    ($head: expr, $($tail: expr),*) => (1 + count_exprs!($($tail),*));
}

macro_rules! recurrence {
    ( $seq: ident [ $ind: ident ]: $sty: ty = $( $inits: expr ),+ => $recur: expr ) => {
        {
            use std::ops::Index;

            const MEM_SIZE: usize = count_exprs!($($inits),+);

            struct Recurrence {
                vec!mem: [$sty; MEM_SIZE],

                pos: usize,
            }

            struct IndexOffset<'a> {
                vec!slice: &'a [$sty; MEM_SIZE],

                offset: usize,
            }

            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;

                fn index(&self, index: usize) -> &$sty {
                    use std::num::Wrapping;

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);

                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }

            impl Iterator for Recurrence {
                type Item = $sty;

                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
                            $recur
                        };

                        {
                            use std::mem::swap;
                            let mut swap_tmp = next_val;
                            (0..MEM_SIZE)
                            .rev()
                            .for_each(|i| swap(&mut swap_tmp, &mut self.mem[i]));
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recuvec!rrence { mem: [$($inits),+],
                 pos: 0 }
        }
    };
}

*/
