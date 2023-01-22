#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SquareStatus {
    Occupied,
    Available,
    Attacked,
}

pub struct Board {
    squares: Vec<Vec<SquareStatus>>,
}

pub struct Row(pub usize);
pub struct Column(pub usize);
pub struct Square {
    pub row: Row,
    pub column: Column,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            squares: vec![vec![SquareStatus::Available; size]; size],
        }
    }

    pub fn status(&self, square: &Square) -> SquareStatus {
        self.squares[square.row.0][square.column.0]
    }

    pub fn set_status(&mut self, square: &Square, status: SquareStatus) {
        self.squares[square.row.0][square.column.0] = status;
    }
}
