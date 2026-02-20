pub trait Piece {
    fn get_name(&self) -> &str;
    fn move_piece(&self) -> ();
    fn get_rank(&self) -> i32;
    fn get_file(&self) -> i32;
    fn set_rank(&mut self, rank: i32);
    fn set_file(&mut self, file: i32);
    fn get_color(&self) -> &str;
    fn set_color(&mut self, color: String) -> ();
    fn get_possible_moves(&self, board: &crate::chessboard::Board) -> Vec<(i32, i32)>;
}