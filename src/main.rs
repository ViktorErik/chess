use macroquad::prelude::*;
use std::collections::HashMap;
mod chessboard;
use chessboard::Board;
mod pieces;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        window_width: 640,
        window_height: 640,
        fullscreen: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {    

    let mut textures = HashMap::new();
    textures.insert("white-pawn".to_string(), load_texture("assets/images/pieces/white-pawn.png").await.unwrap());
    textures.insert("white-rook".to_string(), load_texture("assets/images/pieces/white-rook.png").await.unwrap());
    textures.insert("white-knight".to_string(), load_texture("assets/images/pieces/white-knight.png").await.unwrap());
    textures.insert("white-bishop".to_string(), load_texture("assets/images/pieces/white-bishop.png").await.unwrap());
    textures.insert("white-queen".to_string(), load_texture("assets/images/pieces/white-queen.png").await.unwrap());
    textures.insert("white-king".to_string(), load_texture("assets/images/pieces/white-king.png").await.unwrap());
    textures.insert("black-pawn".to_string(), load_texture("assets/images/pieces/black-pawn.png").await.unwrap());
    textures.insert("black-rook".to_string(), load_texture("assets/images/pieces/black-rook.png").await.unwrap());
    textures.insert("black-knight".to_string(), load_texture("assets/images/pieces/black-knight.png").await.unwrap());
    textures.insert("black-bishop".to_string(), load_texture("assets/images/pieces/black-bishop.png").await.unwrap());
    textures.insert("black-queen".to_string(), load_texture("assets/images/pieces/black-queen.png").await.unwrap());
    textures.insert("black-king".to_string(), load_texture("assets/images/pieces/black-king.png").await.unwrap());

    let mut board = Board::new();
    let mut x: f32;
    let mut y: f32;
    let mouse: MouseButton = MouseButton::Left;

    loop {
        clear_background(BLACK);        

        x = mouse_position().0;
        y = mouse_position().1;

        if is_mouse_button_pressed(mouse) {
            // Handle promotion selection
            if let Some(piece_index) = board.promotion_state {
                let promo_box_width = 80.0;
                let promo_box_height = 80.0;
                let promo_x = (screen_width() - promo_box_width * 4.0) / 2.0;
                let promo_y = screen_height() / 2.0 - promo_box_height / 2.0;
                
                let queen_rect = Rect::new(promo_x, promo_y, promo_box_width, promo_box_height);
                let rook_rect = Rect::new(promo_x + promo_box_width, promo_y, promo_box_width, promo_box_height);
                let bishop_rect = Rect::new(promo_x + promo_box_width * 2.0, promo_y, promo_box_width, promo_box_height);
                let knight_rect = Rect::new(promo_x + promo_box_width * 3.0, promo_y, promo_box_width, promo_box_height);
                
                if queen_rect.contains(Vec2::new(x, y)) {
                    board.promote_piece(piece_index, 'Q');
                } else if rook_rect.contains(Vec2::new(x, y)) {
                    board.promote_piece(piece_index, 'R');
                } else if bishop_rect.contains(Vec2::new(x, y)) {
                    board.promote_piece(piece_index, 'B');
                } else if knight_rect.contains(Vec2::new(x, y)) {
                    board.promote_piece(piece_index, 'N');
                }
            } else {
                board.mark_piece(x, y);
            }
        }
        board.draw_squares();
        board.draw_pieces(&textures);
        
        // Draw promotion UI
        if board.promotion_state.is_some() {
            let promo_box_width = 80.0;
            let promo_box_height = 80.0;
            let promo_x = (screen_width() - promo_box_width * 4.0) / 2.0;
            let promo_y = screen_height() / 2.0 - promo_box_height / 2.0;
            
            // Semi-transparent overlay
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));
            
            // Promotion boxes
            draw_rectangle(promo_x, promo_y, promo_box_width, promo_box_height, BLUE);
            draw_text("Q", promo_x + 30.0, promo_y + 40.0, 30.0, WHITE);
            
            draw_rectangle(promo_x + promo_box_width, promo_y, promo_box_width, promo_box_height, BLUE);
            draw_text("R", promo_x + promo_box_width + 30.0, promo_y + 40.0, 30.0, WHITE);
            
            draw_rectangle(promo_x + promo_box_width * 2.0, promo_y, promo_box_width, promo_box_height, BLUE);
            draw_text("B", promo_x + promo_box_width * 2.0 + 30.0, promo_y + 40.0, 30.0, WHITE);
            
            draw_rectangle(promo_x + promo_box_width * 3.0, promo_y, promo_box_width, promo_box_height, BLUE);
            draw_text("N", promo_x + promo_box_width * 3.0 + 30.0, promo_y + 40.0, 30.0, WHITE);
            
            draw_text("Choose promotion piece:", promo_x, promo_y - 30.0, 20.0, WHITE);
        }            

        // Draw current turn
        let turn_color = if board.get_current_turn() == "white" { GREEN } else { RED };
        draw_text(&format!("{}'s turn", board.get_current_turn()), 10.0, 20.0, 20.0, turn_color);

        // Check for check
        if board.is_in_check(board.get_current_turn()) {
            draw_text("Check!", 10.0, 40.0, 20.0, YELLOW);
        }

        next_frame().await
    }
}

