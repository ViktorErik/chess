use crate::pieces::piece::{Piece};

pub struct King {    
    name: String,
    rank: i32, 
    file: i32,
    pub color: String, 
}


impl King {
    pub fn new(color: &str, rank: i32, file: i32) -> Self {
        Self {
            name: "K".to_string(),
            rank: rank, 
            file: file, 
            color: color.to_string(),
        }
    }
}


impl Piece for King {


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
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
        for &(dr, df) in &directions {
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
        // Castling moves
        if self.rank == 0 && self.color == "white" {
            // White kingside castling
            if !board.white_king_moved && !board.white_rook_h_moved {
                let mut can_castle_kingside = true;
                for i in 5..7 {
                    if board.pieces.iter().any(|p| p.get_rank() == 0 && p.get_file() == i) {
                        can_castle_kingside = false;
                    }
                }
                if can_castle_kingside {
                    moves.push((0, 6)); // Kingside castling move
                }
            }
            // White queenside castling
            if !board.white_king_moved && !board.white_rook_a_moved {
                let mut can_castle_queenside = true;
                for i in 1..4 {
                    if board.pieces.iter().any(|p| p.get_rank() == 0 && p.get_file() == i) {
                        can_castle_queenside = false;
                    }
                }
                if can_castle_queenside {
                    moves.push((0, 2)); // Queenside castling move
                }
            }
        } else if self.rank == 7 && self.color == "black" {
            // Black kingside castling
            if !board.black_king_moved && !board.black_rook_h_moved {
                let mut can_castle_kingside = true;
                for i in 5..7 {
                    if board.pieces.iter().any(|p| p.get_rank() == 7 && p.get_file() == i) {
                        can_castle_kingside = false;
                    }
                }
                if can_castle_kingside {
                    moves.push((7, 6)); // Kingside castling move
                }
            }
            // Black queenside castling
            if !board.black_king_moved && !board.black_rook_a_moved {
                let mut can_castle_queenside = true;
                for i in 1..4 {
                    if board.pieces.iter().any(|p| p.get_rank() == 7 && p.get_file() == i) {
                        can_castle_queenside = false;
                    }
                }
                if can_castle_queenside {
                    moves.push((7, 2)); // Queenside castling move
                }
            }
        }
        moves
    }
     
}