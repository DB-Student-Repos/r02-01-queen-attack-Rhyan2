
#[derive(Debug)]
pub struct ChessPosition{
rank:i32,
file:i32,
}

#[derive(Debug)]
pub struct Queen{
    position:ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (rank>=1 && rank<=8)&&(file>=1 &&file<=8){
            return Some(ChessPosition{file,rank});
        }else {
            return None;
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
    Queen{position}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.rank == other.position.rank{
            return true;
        }
        if self.position.file==other.position.file{
            return true;
        }
        if (self.position.rank - other.position.rank).abs()==(self.position.file - other.position.file).abs(){
            return true;
        }
         false
    

    }
    
}
