extern crate algorithms;

fn main() {
    // let _foo = std::process::Command::new(".\\target\\debug\\array_2d.exe")
    //     .status()
    //     .unwrap();

    println!(
        "{:?}",
        algorithms::leetcode::pacific_atlantic_water_flow::Solution::pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ])
    );
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
