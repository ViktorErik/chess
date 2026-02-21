
struct PieceValues {
    pawn: i32,
    knight: i32,
    bishop: i32,
    rook: i32,
    queen: i32,
    king: i32,
}

impl PieceValues {
    fn new() -> Self {
        PieceValues {
            pawn: 1,
            knight: 3,
            bishop: 3,
            rook: 5,
            queen: 9,
            king: 1000, // Arbitrary high value to prioritize king safety
        }
    }
}

fn evaluate_board(board: &crate::chessboard::Board) -> i32 {
    let values = PieceValues::new();
    let mut score = 0;

    for p in &board.pieces {
        let piece_value = match p.get_name() {
            "P" => values.pawn,
            "N" => values.knight,
            "B" => values.bishop,
            "R" => values.rook,
            "Q" => values.queen,
            "K" => values.king,
            _ => 0,
        };

        if p.get_color() == "white" {
            score += piece_value;
        } else {
            score -= piece_value;
        }
    }        
    score
}

// Alpha-beta minimax. We provide a wrapper `minimax` that calls `minimax_ab` with
// initial alpha/beta values. Moves generated are filtered to legal moves by
// simulating and skipping any that leave the mover's king in check.

fn minimax_ab(board: &crate::chessboard::Board, depth: i32, mut alpha: i32, mut beta: i32, is_maximizing: bool) -> i32 {
    if depth == 0 || board.is_game_over() {
        return evaluate_board(board);
    }

    if is_maximizing {
        let mut value = i32::MIN;
        let color = "white";
        let moves = board.get_all_moves(color);
        // If no generated moves, return evaluation
        if moves.is_empty() {
            return evaluate_board(board);
        }
        for m in moves {
            // simulate
            let mut new_board = board.clone();
            new_board.make_move(m);
            // skip illegal moves that leave own king in check
            if new_board.is_in_check(color) {
                continue;
            }
            let score = minimax_ab(&new_board, depth - 1, alpha, beta, false);
            value = value.max(score);
            alpha = alpha.max(value);
            if alpha >= beta {
                break; // beta cutoff
            }
        }
        value
    } else {
        let mut value = i32::MAX;
        let color = "black";
        let moves = board.get_all_moves(color);
        if moves.is_empty() {
            return evaluate_board(board);
        }
        for m in moves {
            let mut new_board = board.clone();
            new_board.make_move(m);
            if new_board.is_in_check(color) {
                continue;
            }
            let score = minimax_ab(&new_board, depth - 1, alpha, beta, true);
            value = value.min(score);
            beta = beta.min(value);
            if alpha >= beta {
                break; // alpha cutoff
            }
        }
        value
    }
}

pub fn minimax(board: &mut crate::chessboard::Board, depth: i32, is_maximizing: bool) -> i32 {
    // Use reasonably wide initial alpha/beta
    minimax_ab(board, depth, i32::MIN / 4, i32::MAX / 4, is_maximizing)
}
