use macroquad::prelude::*;
use macroquad::texture::DrawTextureParams;
use crate::pieces::{rook::Rook, pawn::Pawn, knight::Knight, bishop::Bishop, king::King, queen::Queen};
use crate::pieces::piece::Piece;


// PIECES ARE: R, N, B, K, Q, P
// RANKS: 1-8
// FILES: A-H

pub struct Board {
    square_size: f32,
    pub pieces: Vec<Box<dyn Piece>>,
    selected_piece: Option<usize>, // index in pieces vector
    possible_moves: Vec<(i32, i32)>,
    current_turn: String,
    pub last_double_pawn_file: Option<i32>,
    pub white_king_moved: bool,
    pub white_rook_a_moved: bool,
    pub white_rook_h_moved: bool,
    pub black_king_moved: bool,
    pub black_rook_a_moved: bool,
    pub black_rook_h_moved: bool,
    pub promotion_state: Option<usize>, // piece index to promote
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
            selected_piece: None,
            possible_moves: Vec::new(),
            current_turn: "white".to_string(),
            last_double_pawn_file: None,
            white_king_moved: false,
            white_rook_a_moved: false,
            white_rook_h_moved: false,
            black_king_moved: false,
            black_rook_a_moved: false,
            black_rook_h_moved: false,
            promotion_state: None,
        }        
    }

    pub fn draw_pieces(&self, textures: &std::collections::HashMap<String, macroquad::texture::Texture2D>) {
        for p in &self.pieces {
            let piece_type = match p.get_name() {
                "P" => "pawn",
                "R" => "rook",
                "N" => "knight",
                "B" => "bishop",
                "Q" => "queen",
                "K" => "king",
                _ => "pawn",
            };
            let key = format!("{}-{}", p.get_color(), piece_type);
            if let Some(tex) = textures.get(&key) {
                let scale = 0.75;
                let w = tex.width() as f32 * scale;
                let h = tex.height() as f32 * scale;
                let x = (p.get_file() as f32) * self.square_size + (self.square_size - w) / 2.0;
                let y = (7 - p.get_rank()) as f32 * self.square_size + (self.square_size - h) / 2.0;
                draw_texture_ex(tex, x, y, WHITE, DrawTextureParams {
                    dest_size: Some(Vec2::new(w, h)),
                    ..Default::default()
                });
            }
        }
    }
   
    pub fn draw_squares(&self) {        
        for r in 0..8 {
            for c in (r%2..8).step_by(2) {
                let y = (7 - r) as f32 * self.square_size;
                draw_rectangle((c as f32) * self.square_size, y, self.square_size, self.square_size, BEIGE);
            }
        }

        for r in 0..8 {
            for c in ((r+1)%2..8).step_by(2) {
                let y = (7 - r) as f32 * self.square_size;
                draw_rectangle((c as f32) * self.square_size, y, self.square_size, self.square_size, WHITE);
            }
        }


        // Highlight possible moves
        for &(rank, file) in &self.possible_moves {
            let y = (7 - rank) as f32 * self.square_size;
            draw_rectangle(file as f32 * self.square_size, y, self.square_size, self.square_size, YELLOW);
        }
        // Highlight selected piece
        if let Some(index) = self.selected_piece {
            let p = &self.pieces[index];
            let y = (7 - p.get_rank()) as f32 * self.square_size;
            draw_rectangle(p.get_file() as f32 * self.square_size, y, self.square_size, self.square_size, BLUE);
        }
    }

    pub fn mark_piece(&mut self, x: f32, y: f32) {
        let file = (x / self.square_size) as i32;
        let rank = 7 - (y / self.square_size) as i32; // since rank 0 is bottom
        if file >= 0 && file < 8 && rank >= 0 && rank < 8 {
            // If a piece is selected, try to move it
            if let Some(selected_index) = self.selected_piece {
                if self.possible_moves.contains(&(rank, file)) {
                    let from_rank = self.pieces[selected_index].get_rank();
                    let from_file = self.pieces[selected_index].get_file();
                    let piece_name = self.pieces[selected_index].get_name().to_string();
                    let piece_color = self.pieces[selected_index].get_color().to_string();
                    let mut adjusted_selected = selected_index;
                    
                    // Handle castling
                    if piece_name == "K" && (file - from_file).abs() == 2 {
                        // Move the rook
                        if file > from_file {
                            // Kingside castling
                            if let Some(rook_index) = self.pieces.iter().position(|p| p.get_rank() == from_rank && p.get_file() == 7 && p.get_name() == "R") {
                                self.pieces[rook_index].set_file(5);
                            }
                        } else {
                            // Queenside castling
                            if let Some(rook_index) = self.pieces.iter().position(|p| p.get_rank() == from_rank && p.get_file() == 0 && p.get_name() == "R") {
                                self.pieces[rook_index].set_file(3);
                            }
                        }
                        // Mark king and rook as moved
                        if piece_color == "white" {
                            self.white_king_moved = true;
                            if file > from_file {
                                self.white_rook_h_moved = true;
                            } else {
                                self.white_rook_a_moved = true;
                            }
                        } else {
                            self.black_king_moved = true;
                            if file > from_file {
                                self.black_rook_h_moved = true;
                            } else {
                                self.black_rook_a_moved = true;
                            }
                        }
                    }
                    
                    // Find and remove captured piece
                    if let Some(captured_index) = self.pieces.iter().position(|p| p.get_rank() == rank && p.get_file() == file) {
                        self.pieces.remove(captured_index);
                        // Adjust selected_index if the captured piece was before it
                        if captured_index < adjusted_selected {
                            adjusted_selected -= 1;
                        }
                    }
                    // Handle en passant capture
                    if piece_name == "P" && file != from_file && self.pieces.iter().find(|p| p.get_rank() == from_rank && p.get_file() == file && p.get_name() == "P").is_none() {
                        // En passant: remove the captured pawn
                        if let Some(en_passant_index) = self.pieces.iter().position(|p| p.get_rank() == from_rank && p.get_file() == file) {
                            self.pieces.remove(en_passant_index);
                            if en_passant_index < adjusted_selected {
                                adjusted_selected -= 1;
                            }
                        }
                    }
                    // Move the piece
                    {
                        let selected_piece = &mut self.pieces[adjusted_selected];
                        selected_piece.set_rank(rank);
                        selected_piece.set_file(file);
                    }
                    
                    // Track if king or rook moved
                    if piece_name == "K" {
                        if piece_color == "white" {
                            self.white_king_moved = true;
                        } else {
                            self.black_king_moved = true;
                        }
                    } else if piece_name == "R" {
                        if piece_color == "white" {
                            if from_file == 0 {
                                self.white_rook_a_moved = true;
                            } else if from_file == 7 {
                                self.white_rook_h_moved = true;
                            }
                        } else {
                            if from_file == 0 {
                                self.black_rook_a_moved = true;
                            } else if from_file == 7 {
                                self.black_rook_h_moved = true;
                            }
                        }
                    }
                    
                    // Set last double pawn move
                    if piece_name == "P" && (rank - from_rank).abs() == 2 {
                        self.last_double_pawn_file = Some(file);
                    } else {
                        self.last_double_pawn_file = None;
                    }
                    // Deselect
                    self.selected_piece = None;
                    self.possible_moves.clear();
                    
                    // Check for pawn promotion
                    if piece_name == "P" && ((piece_color == "white" && rank == 7) || (piece_color == "black" && rank == 0)) {
                        self.promotion_state = Some(adjusted_selected);
                        return;
                    }
                    
                    // Switch turns
                    self.current_turn = if self.current_turn == "white" { "black".to_string() } else { "white".to_string() };
                    return;
                } else {
                    // Clicked elsewhere, deselect
                    self.selected_piece = None;
                    self.possible_moves.clear();
                    return;
                }
            }
            // No piece selected, try to select one
            for (i, p) in self.pieces.iter().enumerate() {
                if p.get_rank() == rank && p.get_file() == file && p.get_color() == self.current_turn {
                    self.selected_piece = Some(i);
                    self.possible_moves = self.get_legal_moves(i);
                    return;
                }
            }
        }
    }
    pub fn is_in_check(&self, color: &str) -> bool {
        // Find king position
        let king_pos = self.pieces.iter().find(|p| p.get_name() == "K" && p.get_color() == color).map(|p| (p.get_rank(), p.get_file()));
        if let Some((kr, kf)) = king_pos {
            // Check if any enemy piece can move to king_pos
            for p in &self.pieces {
                if p.get_color() != color {
                    let moves = p.get_possible_moves(self);
                    if moves.contains(&(kr, kf)) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get_legal_moves(&mut self, piece_index: usize) -> Vec<(i32, i32)> {
        let possible = self.pieces[piece_index].get_possible_moves(self);
        // Always filter moves that would leave the king in check
        let mut legal = Vec::new();
        // Save positions
        let saved_positions: Vec<(i32, i32)> = self.pieces.iter().map(|p| (p.get_rank(), p.get_file())).collect();
        for &(r, f) in &possible {
            let piece_name = self.pieces[piece_index].get_name().to_string();
            let from_file = self.pieces[piece_index].get_file();
            
            // Check for castling and validate castling rules
            if piece_name == "K" && (f - from_file).abs() == 2 {
                // Castling move - check intermediate squares are not in check
                let between_file = (from_file + f) / 2;
                
                // Simulate king on intermediate square
                self.pieces[piece_index].set_rank(r);
                self.pieces[piece_index].set_file(between_file);
                if self.is_in_check(&self.current_turn) {
                    // Intermediate square is in check, can't castle
                    self.pieces[piece_index].set_rank(saved_positions[piece_index].0);
                    self.pieces[piece_index].set_file(saved_positions[piece_index].1);
                    continue;
                }
                
                // Simulate king on final square
                self.pieces[piece_index].set_file(f);
                if self.is_in_check(&self.current_turn) {
                    // Final square is in check, can't castle
                    self.pieces[piece_index].set_rank(saved_positions[piece_index].0);
                    self.pieces[piece_index].set_file(saved_positions[piece_index].1);
                    continue;
                }
                
                legal.push((r, f));
                self.pieces[piece_index].set_rank(saved_positions[piece_index].0);
                self.pieces[piece_index].set_file(saved_positions[piece_index].1);
            } else {
                // Regular move - simulate and check
                // Simulate move
                self.pieces[piece_index].set_rank(r);
                self.pieces[piece_index].set_file(f);
                // Remove captured if any
                let mut captured_idx = None;
                for (i, p) in self.pieces.iter().enumerate() {
                    if i != piece_index && p.get_rank() == r && p.get_file() == f {
                        captured_idx = Some(i);
                        // Temporarily move captured away
                        self.pieces[i].set_rank(-1);
                        self.pieces[i].set_file(-1);
                        break;
                    }
                }
                // Check if king is in check after move
                if !self.is_in_check(&self.current_turn) {
                    legal.push((r, f));
                }
                // Restore
                if let Some(idx) = captured_idx {
                    self.pieces[idx].set_rank(saved_positions[idx].0);
                    self.pieces[idx].set_file(saved_positions[idx].1);
                }
                self.pieces[piece_index].set_rank(saved_positions[piece_index].0);
                self.pieces[piece_index].set_file(saved_positions[piece_index].1);
            }
        }
        legal
    }

    pub fn get_current_turn(&self) -> &str {
        &self.current_turn
    }

    pub fn promote_piece(&mut self, piece_index: usize, piece_type: char) {
        let color = self.pieces[piece_index].get_color().to_string();
        let rank = self.pieces[piece_index].get_rank();
        let file = self.pieces[piece_index].get_file();
        
        // Remove the pawn
        self.pieces.remove(piece_index);
        
        // Add the new piece
        let new_piece: Box<dyn Piece> = match piece_type {
            'Q' => Box::new(Queen::new(&color, rank, file)),
            'R' => Box::new(Rook::new(&color, rank, file)),
            'B' => Box::new(Bishop::new(&color, rank, file)),
            'N' => Box::new(Knight::new(&color, rank, file)),
            _ => Box::new(Queen::new(&color, rank, file)),
        };
        self.pieces.push(new_piece);
        
        // Clear promotion state and switch turns
        self.promotion_state = None;
        self.selected_piece = None;
        self.possible_moves.clear();
        self.current_turn = if self.current_turn == "white" { "black".to_string() } else { "white".to_string() };
    }
}