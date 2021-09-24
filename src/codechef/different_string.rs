fn flip_bit(bit: char) -> char {
    if bit == '1' {
        '0'
    } else if bit == '0' {
        '1'
    } else {
        panic!("Invalid Bit type");
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines().map(Result::unwrap);

    let num_test_cases: u32 = input_lines.next().unwrap().parse()?;
    for _ in 0..num_test_cases {
        let str_len: usize = input_lines.next().unwrap().parse()?;
        let mut diff_string = "".to_string();

        for j in 0..str_len {
            let bit_string = input_lines.next().unwrap();
            diff_string.push(flip_bit(bit_string.as_bytes()[j] as char));
        }

        println!("{}", diff_string);
    }
    Ok(())
}
