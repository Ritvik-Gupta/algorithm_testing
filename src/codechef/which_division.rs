use text_io::read;

fn algorithm() {
    let rating: u32 = read!();
    println!(
        "{}",
        if rating >= 2000 {
            1
        } else if rating >= 1600 {
            2
        } else {
            3
        }
    );
}

use crate::services::Returns;

pub fn main() -> Returns {
    let num_testcases: u32 = read!();
    for _ in 0..num_testcases {
        algorithm();
    }
    Ok(())
}
