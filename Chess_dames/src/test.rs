use rand::Rng;

struct ChessBoard {
    size: u32,
} 
struct Point {
    x: u32,
    y: u32,
}

fn find_potential_dames(board: &ChessBoard) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut points: Vec<Point> = Vec::<Point>::new();
    let mut n = board.size;
    points.push(new Point{x:rng.gen_range(0..board.size), y:rng.gen_range(0..board.size)});
    //for i in 0..board.size
      //  for j in 0..board.size
        
}
fn main() {

  
}