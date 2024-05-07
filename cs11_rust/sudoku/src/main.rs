use sudoku::Sudoku;

fn main() {
    let mut b = Sudoku::load("boards/board_hard2.txt").unwrap();
    println!("{}", b);

    b.solve();
    println!("{}", b);
}