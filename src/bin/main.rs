extern crate algorithms;

fn main() {
    // let _foo = std::process::Command::new(".\\target\\debug\\array_2d.exe")
    //     .status()
    //     .unwrap();

    let mut trie = algorithms::trie::link::TrieLink::new();
    trie.insert("a");
    trie.insert("ab");
    trie.insert("bc");
    trie.insert("bca");
    trie.insert("bab");
    trie.insert("c");
    trie.insert("caa");

    {
        let mut trie_2 = trie.clone();
        let ac_trie = trie_2.lock();
        println!("\n\n\n");
        trie.print();
        ac_trie.match_against("abccabxfwa");
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
