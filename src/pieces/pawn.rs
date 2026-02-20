use crate::pieces::piece::{Piece};

pub struct Pawn {    
    name: String,
    rank: i32, 
    file: i32,
    color: String, 
}

impl Pawn {
    pub fn new(color: &str, rank: i32, file: i32) -> Self {
        Self {
            name: "P".to_string(),
            rank: rank, 
            file: file, 
            color: color.to_string(),
        }
    }
}

impl Piece for Pawn {


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
        let direction = if self.color == "white" { 1 } else { -1 };
        let new_rank = self.rank + direction;
        if new_rank >= 0 && new_rank < 8 {
            // Check if square is empty
            let mut occupied = false;
            for p in &board.pieces {
                if p.get_rank() == new_rank && p.get_file() == self.file {
                    occupied = true;
                    break;
                }
            }
            if !occupied {
                moves.push((new_rank, self.file));
                // Double move from starting position
                if (self.color == "white" && self.rank == 1) || (self.color == "black" && self.rank == 6) {
                    let double_rank = self.rank + 2 * direction;
                    if double_rank >= 0 && double_rank < 8 {
                        let mut occupied_double = false;
                        for p in &board.pieces {
                            if p.get_rank() == double_rank && p.get_file() == self.file {
                                occupied_double = true;
                                break;
                            }
                        }
                        if !occupied_double {
                            moves.push((double_rank, self.file));
                        }
                    }
                }
            }
        }
        // Captures
        for &file_offset in &[-1, 1] {
            let capture_file = self.file + file_offset;
            if capture_file >= 0 && capture_file < 8 {
                let capture_rank = self.rank + direction;
                if capture_rank >= 0 && capture_rank < 8 {
                    for p in &board.pieces {
                        if p.get_rank() == capture_rank && p.get_file() == capture_file && p.get_color() != self.color {
                            moves.push((capture_rank, capture_file));
                            break;
                        }
                    }
                }
            }
        }
        // En passant
        if let Some(en_file) = board.last_double_pawn_file {
            if (self.color == "white" && self.rank == 4) || (self.color == "black" && self.rank == 3) {
                if (en_file == self.file - 1) || (en_file == self.file + 1) {
                    // Check if there's an enemy pawn at the en passant rank and file
                    let en_rank = if self.color == "white" { 4 } else { 3 };
                    for p in &board.pieces {
                        if p.get_rank() == en_rank && p.get_file() == en_file && p.get_color() != self.color && p.get_name() == "P" {
                            let move_rank = if self.color == "white" { 5 } else { 2 };
                            moves.push((move_rank, en_file));
                            break;
                        }
                    }
                }
            }
        }
        moves
    }
     
}