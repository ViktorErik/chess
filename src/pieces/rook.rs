use crate::pieces::piece::{Piece};

pub struct Rook {    
    name: String,
    rank: i32, 
    file: i32,
    color: String, 
}


impl Rook {
    pub fn new(color: &str, rank: i32, file: i32) -> Self {
        Self {
            name: "R".to_string(),
            rank: rank, 
            file: file, 
            color: color.to_string(),
        }
    }
}


impl Piece for Rook {

    
    fn get_color(&self) -> &str {
        return &self.color
    }


    fn get_rank(&self) -> i32 {
        return self.rank;
    }

    fn get_file(&self) -> i32 {
        return self.file;
    }

    fn move_piece(&self) -> () {

    }

    fn get_name(&self) -> &str {
        return &(self.name);
    }
     
}