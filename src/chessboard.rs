use macroquad::prelude::*;
use crate::pieces::{rook::Rook, pawn::Pawn, knight::Knight, bishop::Bishop, king::King, queen::Queen};
use crate::pieces::piece::Piece;


// PIECES ARE: R, N, B, K, Q, P
// RANKS: 1-8
// FILES: A-H

pub struct Board {
    square_size: f32,
    pieces: Vec<Box<dyn Piece>>,
}

impl Board {

    pub fn new() -> Self {
        Self {
            square_size: screen_width() / 8.0,
            // pieces: vec!["R".to_string()],
            pieces: vec![
                Box::new(Rook::new("white", 0, 0)), 
                Box::new(Knight::new("white", 0, 1)), 
                Box::new(Bishop::new("white", 0, 2)), 
                Box::new(Queen::new("white", 0, 3)), 
                Box::new(King::new("white", 0, 4)), 
                Box::new(Bishop::new("white", 0, 5)), 
                Box::new(Knight::new("white", 0, 6)), 
                Box::new(Rook::new("white", 0, 7)), 

                Box::new(Pawn::new("white", 1, 0)),
                Box::new(Pawn::new("white", 1, 1)),
                Box::new(Pawn::new("white", 1, 2)),
                Box::new(Pawn::new("white", 1, 3)),
                Box::new(Pawn::new("white", 1, 4)),
                Box::new(Pawn::new("white", 1, 5)),
                Box::new(Pawn::new("white", 1, 6)),
                Box::new(Pawn::new("white", 1, 7)),

                Box::new(Rook::new("black", 7, 0)), 
                Box::new(Knight::new("black", 7, 1)), 
                Box::new(Bishop::new("black", 7, 2)), 
                Box::new(Queen::new("black", 7, 3)), 
                Box::new(King::new("black", 7, 4)), 
                Box::new(Bishop::new("black", 7, 5)), 
                Box::new(Knight::new("black", 7, 6)), 
                Box::new(Rook::new("black", 7, 7)), 

                Box::new(Pawn::new("black", 6, 0)),
                Box::new(Pawn::new("black", 6, 1)),
                Box::new(Pawn::new("black", 6, 2)),
                Box::new(Pawn::new("black", 6, 3)),
                Box::new(Pawn::new("black", 6, 4)),
                Box::new(Pawn::new("black", 6, 5)),
                Box::new(Pawn::new("black", 6, 6)),
                Box::new(Pawn::new("black", 6, 7)),
            ],
        }        
    }

    pub fn draw_pieces(&self) {
        for p in &self.pieces {

            let color = if p.get_color() == "white" {
                GREEN
            } else {
                RED
            };

            draw_text(&p.get_name(), 
            (self.square_size / 2.0 + self.square_size * p.get_file() as f32), 
            (screen_height() - self.square_size / 2.0 - self.square_size * p.get_rank() as f32), 
            30.0, color);
        }
    }
   
    pub fn draw_squares(&self) {        
        for r in 0..8 {
            for c in (r%2..8).step_by(2) {
                draw_rectangle((c as f32) * self.square_size, (r as f32) * self.square_size, self.square_size, self.square_size, WHITE);
            }
        }
    }
}