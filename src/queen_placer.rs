#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Row(pub usize);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Column(pub usize);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Square {
    pub row: Row,
    pub column: Column,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum SquareStatus {
    Occupied,
    Available,
    Attacked,
}

struct QueenPlacer {
    queen_positions: Vec<Square>,
    size: usize,
}

impl QueenPlacer {
    fn new(size: usize) -> Self {
        QueenPlacer {
            queen_positions: Vec::new(),
            size,
        }
    }

    fn place_queens(&mut self) -> Vec<Square> {
        Vec::new()
    }

    fn queens_left_to_add(&self) -> bool {
        self.queen_positions.len() < self.size
    }

    fn place_queen(&self, square: Square) {
        assert!(self.queens_left_to_add());
        assert!(square.row == Row(self.queen_positions.len()));
    }
}

pub fn place_queens(size: usize) -> Vec<Square> {
    let mut placer = QueenPlacer::new(size);
    placer.place_queens()
}
