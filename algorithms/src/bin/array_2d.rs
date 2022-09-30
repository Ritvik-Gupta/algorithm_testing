use algorithms::array_2d::Array2D;
use services::Returns;

fn main() -> Returns {
    let ref mut matrix = Array2D::with_generator((3, 5), |vec_pos| format!("{:?}", vec_pos));
    println!("{} x {}", matrix.num_rows(), matrix.num_cols());
    matrix[(1, 1)] = "abc".to_string();
    println!("{:?}", matrix.iter_mut().collect::<Vec<_>>());
    println!("{:?}", matrix.iter().collect::<Vec<_>>());

    println!("{:?}", matrix.row_iter(0).collect::<Vec<_>>());
    println!("{:?}", matrix.row_iter(1).collect::<Vec<_>>());
    println!("{:?}", matrix.row_iter(2).collect::<Vec<_>>());

    Ok(())
}
