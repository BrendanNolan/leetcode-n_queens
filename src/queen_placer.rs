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

impl Square {
    fn new(row: Row, column: Column) -> Self {
        Square { row, column }
    }
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
    fn new(size: usize) -> Self {
        QueenPlacer {
            queen_positions: Vec::new(),
            size,
            columns_available: (0..size).map(Column).collect(),
        }
    }

    fn place_queens(mut self) -> Vec<Square> {
        // while let Some(row) = self.next_row_for_placement() {}
        Vec::new()
    }

    fn next_row_for_placement(&self) -> Option<Row> {
        let rows_filled = self.queen_positions.len();
        if rows_filled == self.size {
            None
        } else {
            Some(Row(self.queen_positions.len()))
        }
    }

    fn place_queen(&mut self, square: &Square) {
        assert!(square.row == Row(self.queen_positions.len()));
        assert!(self.can_place_queen_at(square));
        self.columns_available.remove(&square.column);
        self.queen_positions.push(*square);
    }

    fn can_place_queen_at(&self, square: &Square) -> bool {
        self.queen_positions
            .iter()
            .any(|existing_queen| queens_tolerate_each_other(existing_queen, square))
    }

    fn remove_last_queen(&mut self) -> Square {
        assert!(!self.queen_positions.is_empty());
        let square = self.queen_positions.pop().unwrap();
        self.columns_available.insert(square.column);
        square
    }
}

fn queens_tolerate_each_other(queen_a: &Square, queen_b: &Square) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_queens_attack() {
        let queen_a = Square::new(Row(0), Column(1));
        let queen_b = Square::new(Row(2), Column(3));
        assert!(queens_tolerate_each_other(&queen_a, &queen_b));
    }
}
