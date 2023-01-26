use std::collections::HashSet;

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub struct Row(pub usize);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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
    columns_available: HashSet<Column>,
}

impl QueenPlacer {
    fn place_queens(mut self) -> Vec<Square> {
        Vec::new()
    }

    fn new(size: usize) -> Self {
        QueenPlacer {
            queen_positions: Vec::new(),
            size,
            columns_available: (0..size).map(Column).collect(),
        }
    }

    fn next_row_for_placement(&self) -> Option<Row> {
        let rows_filled = self.queen_positions.len();
        if rows_filled == self.size {
            None
        } else {
            Some(Row(self.queen_positions.len()))
        }
    }

    fn attempt_place_queen(&mut self, square: &Square) -> bool {
        assert!(square.row == Row(self.queen_positions.len()));
        self.columns_available.remove(&square.column);
        true
    }
}

fn queens_attack(queen_a: &Square, queen_b: &Square) -> bool {
    if queen_a.row == queen_b.row || queen_a.column == queen_b.column {
        return true;
    }
    (queen_a.row.0 as i32 - queen_b.row.0 as i32).abs()
        == (queen_a.column.0 as i32 - queen_b.column.0 as i32).abs()
}

pub fn place_queens(size: usize) -> Vec<Square> {
    let placer = QueenPlacer::new(size);
    placer.place_queens()
}
