use rand::Rng;
use std::io;

#[derive(Debug)]
struct Mypaths {
    rows: Vec<i32>,
    cols: Vec<i32>,
    diag: Vec<i32>,
    anti_diag: Vec<i32>,
}

fn is_safe(board: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    // Check the left side of the current row
    for i in 0..col {
        if board[row][i] != 0 {
            return false;
        }
    }
    // Check the right side of the current column
    for i in 0..row {
        if board[i][col] != 0 {
            return false;
        }
    }
    // Check upper diagonal on the left side
    for i in (0..row).rev() {
        if col >= i && board[i][col - i] != 0 {
            return false;
        }
    }

    // Check lower diagonal on the left side
    let n = board.len();
    for i in 1..n {
        if row + i < n && col >= i && board[row + i][col - i] != 0 {
            return false;
        }
    }

    true
}

fn find_max_queens(
    board: &mut Vec<Vec<i32>>,
    col: usize,
    max_solutions: &mut usize,
    max_positions: &mut Vec<(usize, usize)>,
) {
    let n = board.len();
    if col == n {
        let mut num_queens = 0;
        let mut positions = Vec::new();

        for row in 0..n {
            if board[row][col - 1] != 0 {
                num_queens += 1;
                positions.push((row, col - 1));
            }
        }

        if num_queens > *max_solutions {
            *max_solutions = num_queens;
            max_positions.clear();
            max_positions.extend_from_slice(&positions);
        }

        return;
    }

    for row in 0..n {
        if is_safe(board, row, col) {
            //board[row][col] = 1;
            find_max_queens(board, col + 1, max_solutions, max_positions);
            board[row][col] = 0;
        }
    }
}

fn initialisation(n: usize) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; n]; n];
    let mut current_value = 1; // Valeur de départ

    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = current_value;
            current_value += 1; // Incrémente la valeur pour la prochaine cellule
        }
    }
    matrix
}

fn pick_positions(n: usize) -> Vec<usize> {
    let mut positions = Vec::new();
    positions.push(rand::thread_rng().gen_range(0, n));
    positions.push(rand::thread_rng().gen_range(0, n));
    positions
}

fn diag_search(chessboard: Vec<Vec<i32>>, n : usize) -> u32 {
   
    let mut compteur: u32 = 0;
    //Initialisation avec un point de l'échiquier
    let mut point: Vec<usize> = pick_positions(n);
    let row = point[0];
    let col = point[1];
   // Afficher l'échiquier original
   println!("Échiquier original :");
   for row in &chessboard {
       println!("{:?}", row);
   }

   // Obtenir les diagonales principales
   let mut main_diagonals: Vec<Vec<i32>> = Vec::new();
   for i in 0..n {
       let mut diagonal: Vec<i32> = Vec::new();
       for j in 0..n {
           if i + j < n {
               diagonal.push(chessboard[i + j][j]);
           }
       }
      let mut diagonal2: Vec<i32> = Vec::new();
       for j in 0..n {
           if i + j < n {
               diagonal2.push(chessboard[j][i+j]);
           }
       }
       
           if !diagonal.is_empty() && !main_diagonals.contains(&diagonal) {
               main_diagonals.push(diagonal);
           }
           if !diagonal2.is_empty() && !main_diagonals.contains(&diagonal2) {
               main_diagonals.push(diagonal2);
           }
       
   }

   // Obtenir les anti-diagonales
   let mut anti_diagonals: Vec<Vec<i32>> = Vec::new();
   for i in 0..n {
       let mut diagonal: Vec<i32> = Vec::new();
       for j in 0..n {
           if i >= j {
               diagonal.push(chessboard[i - j][j]);
           }
       }
       let mut diagonal2: Vec<i32> = Vec::new();
       for j in 0..n {
           if i >= j {
               diagonal2.push(chessboard[n-i+j-1][n-j-1]);
           }
       }
       
           if !diagonal.is_empty() &&!anti_diagonals.contains(&diagonal) {
               anti_diagonals.push(diagonal);
           }
           if !diagonal2.is_empty() && !anti_diagonals.contains(&diagonal2) {
               anti_diagonals.push(diagonal2);
           }
       
   }

   // Afficher les diagonales principales
   println!("Diagonales principales :");
   let thediag1: Vec<i32>;
   for diag in &main_diagonals {
       if diag.contains(&chessboard[row][col]){
           //thediag1 = diag ;
       }
           
       println!("{:?}", diag);
   }

   // Afficher les anti-diagonales
   println!("Anti-diagonales :");
   let thediag2: Vec<i32>;
   for diag in &anti_diagonals {
       if diag.contains(&chessboard[row][col]){
           //thediag2 = diag;
       }
           
       println!("{:?}", diag);
   }

   /*let mut meslignes =  Mypaths{
      rows : chessboard[row],
      cols : chessboard[col],
      diag : thediag1,
      anti_diag : thediag2,
   };*/
   
  // meslignes
  compteur

}

fn main() {

    
    println!("Quelle la taille de l'échiquier désirez-vous ?");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Échec de la lecture de l'entrée utilisateur");
        let n: usize = n.trim().parse().expect("Veuillez entrer un nombre !");
        //n = n.try_into().unwrap();
    //let mut n = 11; // Change this to the desired board size (e.g., 8x8)
    let mut board = vec![vec![false; n]; n];
    let mut chessboard: Vec<Vec<i32>> = vec![vec![0; n]; n];
    let mut max_solutions = 0;
    let mut max_positions = Vec::new();    

    println!("Votre nombre : {}", n);
    //initialisation de l'echiquier
    chessboard = initialisation(n);
    println!("{:?}", chessboard);
    
    //let mut counter = diag_search(&mut chessboard, n);
    //println!("les deplacements {:?}", deplacements);
    find_max_queens(&mut chessboard, 0, &mut max_solutions, &mut max_positions);

    println!("Maximum number of queens on a {}x{} board: {}", n, n, max_solutions);
    println!("Queen positions: {:?}", max_positions);
   

    
}


