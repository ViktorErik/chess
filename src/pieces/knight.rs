use crate::pieces::piece::{Piece};

pub struct Knight {    
    name: String,
    rank: i32, 
    file: i32,
    pub color: String, 
}


impl Knight {
    pub fn new(color: &str, rank: i32, file: i32) -> Self {
        Self {
            name: "N".to_string(),
            rank: rank, 
            file: file, 
            color: color.to_string(),
        }
    }
}


impl Piece for Knight {

    fn set_color(&mut self, color: String) -> () {
        self.color = color;
    }


    fn get_color(&self) -> &str {
        return &self.color
    }

    
    fn get_rank(&self) -> i32 {
        return self.rank;
    }

    fn get_file(&self) -> i32 {
        return self.file;
    }

    fn set_rank(&mut self, rank: i32) {
        self.rank = rank;
    }

    fn set_file(&mut self, file: i32) {
        self.file = file;
    }

    fn move_piece(&self) -> () {

    }

    fn get_name(&self) -> &str {
        return &(self.name);
    }

    fn get_possible_moves(&self, board: &crate::chessboard::Board) -> Vec<(i32, i32)> {
        let mut moves = Vec::new();
        let knight_moves = [
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2)
        ];
        for &(dr, df) in &knight_moves {
            let r = self.rank + dr;
            let f = self.file + df;
            if r >= 0 && r < 8 && f >= 0 && f < 8 {
                let mut occupied = false;
                let mut capture = false;
                for p in &board.pieces {
                    if p.get_rank() == r && p.get_file() == f {
                        occupied = true;
                        if p.get_color() != self.color {
                            capture = true;
                        }
                        break;
                    }
                }
                if !occupied || capture {
                    moves.push((r, f));
                }
            }
        }
        moves
    }
     
}