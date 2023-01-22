use crate::board::{Board, Column, Row, Square};

mod board;

fn main() {
    let b = Board::new(3);
    let status = b.status(&Square {
        row: Row(0),
        column: Column(0),
    });
    println!("Square is available: {status:#?}");
}
