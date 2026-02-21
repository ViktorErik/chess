use macroquad::prelude::*;
use std::collections::HashMap;
mod chessboard;
use chessboard::Board;
mod pieces;
mod AI;
use AI::minimax::minimax;
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
    let mouse_right: MouseButton = MouseButton::Right;
    let mut auto_play = false; // toggle automatic play
    let mut last_move_time = get_time();
    let move_delay = 0.6f64; // seconds between automatic moves
    let mut auto_move_history: Vec<((i32, i32), (i32, i32))> = Vec::new();
    let mut position_history: Vec<String> = Vec::new();
    // record initial position
    position_history.push(board.position_key());

    loop {
        clear_background(BLACK);        

        // Toggle autoplay with the `P` key
        if is_key_pressed(KeyCode::P) {
            auto_play = !auto_play;
            println!("Auto play: {}", auto_play);
            last_move_time = get_time();
            // Reset history when toggling autoplay to avoid stale repetition data
            auto_move_history.clear();
            position_history.clear();
            position_history.push(board.position_key());
        }

        // Automatic play loop: when enabled, pick and apply best move every `move_delay` seconds
        if auto_play && get_time() - last_move_time > move_delay && board.promotion_state.is_none() && !board.is_game_over() {
            let color = board.get_current_turn().to_string();
            let moves = board.get_all_moves(&color);
            if !moves.is_empty() {
                let mut best_move = moves[0];
                // Track whether any candidate was evaluated (not skipped due to repetition)
                let mut evaluated_any = false;
                let mut skipped: Vec<((i32, i32), (i32, i32))> = Vec::new();
                // parity determines which indices in history correspond to this player
                let parity = if color == "white" { 0 } else { 1 };

                if color == "white" {
                    let mut best_score = i32::MIN;
                    for m in moves {
                        // detect strict alternating repetition ABABAB (three repeats)
                        if auto_move_history.len() >= 6 {
                            let len = auto_move_history.len();
                            let a = auto_move_history[len-6];
                            let b = auto_move_history[len-5];
                            let c = auto_move_history[len-4];
                            let d = auto_move_history[len-3];
                            let e = auto_move_history[len-2];
                            let f = auto_move_history[len-1];
                            if a == c && c == e && b == d && d == f {
                                // next white move would repeat `a` — avoid it
                                if m == a {
                                    skipped.push(m);
                                    continue;
                                }
                            }
                        }
                        // Count prior consecutive identical moves by this side
                        let mut same_count = 0;
                        for (i, past) in auto_move_history.iter().enumerate().rev() {
                            if i % 2 == parity {
                                if *past == m { same_count += 1; } else { break; }
                            }
                        }
                        if same_count >= 2 {
                            skipped.push(m);
                            continue;
                        }

                        let mut new_board = board.clone();
                        new_board.make_move(m);
                        // skip moves that would create a third occurrence of the same position
                        let key = new_board.position_key();
                        let mut count = 0;
                        for pk in &position_history { if pk == &key { count += 1; } }
                        if count >= 2 { skipped.push(m); continue; }
                        if new_board.is_in_check("white") { continue; }
                        let score = minimax(&mut new_board, 2, false);
                        evaluated_any = true;
                        if score > best_score {
                            best_score = score;
                            best_move = m;
                        }
                    }
                    if !evaluated_any {
                        // fallback: try skipped moves
                        for m in skipped.iter().cloned() {
                            let mut new_board = board.clone();
                            new_board.make_move(m);
                            if new_board.is_in_check("white") { continue; }
                            best_move = m;
                            break;
                        }
                    }
                } else {
                    let mut best_score = i32::MAX;
                    for m in moves {
                        // detect strict alternating repetition ABABAB (three repeats)
                        if auto_move_history.len() >= 6 {
                            let len = auto_move_history.len();
                            let a = auto_move_history[len-6];
                            let b = auto_move_history[len-5];
                            let c = auto_move_history[len-4];
                            let d = auto_move_history[len-3];
                            let e = auto_move_history[len-2];
                            let f = auto_move_history[len-1];
                            if a == c && c == e && b == d && d == f {
                                // next black move would repeat `b` — avoid it
                                if m == b {
                                    skipped.push(m);
                                    continue;
                                }
                            }
                        }
                        
                        let mut same_count = 0;
                        for (i, past) in auto_move_history.iter().enumerate().rev() {
                            if i % 2 == parity {
                                if *past == m { same_count += 1; } else { break; }
                            }
                        }
                        if same_count >= 2 {
                            skipped.push(m);
                            continue;
                        }

                        let mut new_board = board.clone();
                        new_board.make_move(m);
                        let key = new_board.position_key();
                        let mut count = 0;
                        for pk in &position_history { if pk == &key { count += 1; } }
                        if count >= 2 { skipped.push(m); continue; }
                        if new_board.is_in_check("black") { continue; }
                        let score = minimax(&mut new_board, 2, true);
                        evaluated_any = true;
                        if score < best_score {
                            best_score = score;
                            best_move = m;
                        }
                    }
                    if !evaluated_any {
                        for m in skipped.iter().cloned() {
                            let mut new_board = board.clone();
                            new_board.make_move(m);
                            if new_board.is_in_check("black") { continue; }
                            best_move = m;
                            break;
                        }
                    }
                }
                board.make_move(best_move);
                auto_move_history.push(best_move);
                // record resulting position
                position_history.push(board.position_key());
                // keep history bounded
                if auto_move_history.len() > 64 {
                    auto_move_history.remove(0);
                }
                println!("Auto {:?} played", best_move);
                last_move_time = get_time();
            }
        }

        if is_mouse_button_pressed(mouse_right) {
            let is_white_turn = board.get_current_turn().to_string() == "white";
            let color = board.get_current_turn().to_string();
            let moves = board.get_all_moves(&color);
            if moves.is_empty() {
                println!("No legal moves for {}", color);
            } else {
                let mut best_move = moves[0];
                if is_white_turn {
                    let mut best_score = i32::MIN;
                    for m in moves {
                        let mut new_board = board.clone();
                        new_board.make_move(m);
                        // skip illegal moves that leave white in check
                        if new_board.is_in_check("white") {
                            continue;
                        }
                        let score = minimax(&mut new_board, 3, false);
                        if score > best_score {
                            best_score = score;
                            best_move = m;
                        }
                    }
                    println!("Best move for white: {:?} -> score {}", best_move, best_score);
                } else {
                    let mut best_score = i32::MAX;
                    for m in moves {
                        let mut new_board = board.clone();
                        new_board.make_move(m);
                        // skip illegal moves that leave black in check
                        if new_board.is_in_check("black") {
                            continue;
                        }
                        let score = minimax(&mut new_board, 3, true);
                        if score < best_score {
                            best_score = score;
                            best_move = m;
                        }
                    }
                    println!("Best move for black: {:?} -> score {}", best_move, best_score);
                }
            }
        }

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

