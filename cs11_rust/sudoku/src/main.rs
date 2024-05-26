use sudoku::Sudoku;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];

    let mut b = Sudoku::load(file_name).unwrap();
    println!("{}", b);

    b.solve();
    println!("{}", b);
}