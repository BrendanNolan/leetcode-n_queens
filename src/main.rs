mod queen_placer;

fn main() {
    let queen_positions = queen_placer::place_queens(4);
    println!("{queen_positions:#?}")
}
