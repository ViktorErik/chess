pub trait Piece {
    fn get_name(&self) -> &str;
    fn move_piece(&self) -> ();
    fn get_rank(&self) -> i32;
    fn get_file(&self) -> i32;    
    fn get_color(&self) -> &str;
}