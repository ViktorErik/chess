use macroquad::prelude::*;
mod chessboard;
use chessboard::Board;
mod pieces;
use std::io::{self, Write};

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        window_width: 320,
        window_height: 320,
        fullscreen: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {    

    let board = Board::new();
    let mut input = String::new();

    loop {
        clear_background(BLACK);        



        board.draw_squares();
        board.draw_pieces();
        

         let mut input = String::new();
    print!("Enter move: ");
    io::stdout().flush().unwrap(); // important!
    io::stdin().read_line(&mut input).unwrap();

    println!("You entered: {}", input.trim());

        next_frame().await
    }
}




// draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
// draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
// draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);