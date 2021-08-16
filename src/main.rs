#[macro_use]
extern crate text_io;

mod binary_tree;
mod codechef;
mod fraction;
mod leetcode;

// use binary_tree::{
//     binary_child::BinaryChild::{LEFT, RIGHT},
//     iter::IterOrder::{INFIX, POSTFIX, PREFIX},
//     BinaryTree,
// };

// fn main() {
//     let mut binary_tree = BinaryTree::new(1);
//     binary_tree <<= 2;
//     binary_tree <<= 6;
//     binary_tree >>= 3;

//     let a = &mut binary_tree[LEFT];
//     a.set(LEFT, 4);
//     a.set(RIGHT, 7);

//     let b = &mut binary_tree[RIGHT];
//     b.set(RIGHT, 5);

//     println!("{:?}", binary_tree.inorder());
//     println!("{:?}", binary_tree.preorder());
//     println!("{:?}", binary_tree.postorder());

//     println!("{:?}", binary_tree.iter(INFIX).collect::<Vec<_>>());
//     println!("{:?}", binary_tree.iter(PREFIX).collect::<Vec<_>>());
//     println!("{:?}", binary_tree.iter(POSTFIX).collect::<Vec<_>>());
// }

fn main() {
    use fraction::AsFraction;
    use fraction::Fraction;

    let f1 = Fraction::new(1, 2);
    let f2 = 5.fr();

    println!("{}, {}, {}, {}, {}, {}", f1, f2, !f1, f1 + f2, f1 * f2, -f2);
}
