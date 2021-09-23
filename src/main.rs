#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_variables)]

#[macro_use(read)]
extern crate text_io;

mod array_2d;
mod binary_tree;
mod codechef;
mod double_linked_list;
mod fraction;
mod leetcode;
mod matrix;
mod services;

fn main() {
    println!(
        "{}",
        leetcode::time_needed_to_inform_all_employees::Solution::num_of_minutes(
            8,
            0,
            vec![-1, 5, 0, 6, 7, 0, 0, 0],
            vec![89, 0, 0, 0, 0, 523, 241, 519],
        )
    );
}

// use array_2d::Array2D;
// use services::Returns;

// fn main() -> Returns {
//     let ref mut matrix = Array2D::with_generator((3, 5), |vec_pos| format!("{:?}", vec_pos));
//     println!("{} x {}", matrix.num_rows(), matrix.num_cols());
//     matrix[(1, 1)] = "abc".to_string();
//     println!("{:?}", matrix.iter_mut().collect::<Vec<_>>());
//     println!("{:?}", matrix.iter().collect::<Vec<_>>());

//     println!("{:?}", matrix.row_iter(0).collect::<Vec<_>>());
//     println!("{:?}", matrix.row_iter(1).collect::<Vec<_>>());
//     println!("{:?}", matrix.row_iter(2).collect::<Vec<_>>());

//     Ok(())
// }

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
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }

            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
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

            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };
}

*/
