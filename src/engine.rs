const ROW: usize = 8;

pub struct State {
    pub board: [i8; 64],
    pub turn: bool,
    pub depth: u8,
    pub score: f32,
}

impl State {
    pub fn new() -> State {
        //! Convenience function to create a new board with all pieces in the default position.
        //! 
        //! Takes no arguments.
        
        State {
            board: [-4, -2, -3, -5, -6, -3, -2, -4,
                    -1, -1, -1, -1, -1, -1, -1, -1,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                    1, 1, 1, 1, 1, 1, 1, 1,
                    4, 2, 3, 5, 6, 3, 2, 4],
            turn: true,
            depth: 0,
            score: 0f32,
        }
    }

    fn init(board: [i8; 64], turn: bool, depth: u8, score: f32) -> State {
        State {
            board,
            turn,
            depth,
            score
        }
    }

    fn copy(&self) -> State {
        return State {
            board: self.board,
            turn:  self.turn,
            depth: self.depth,
            score: self.score,
        }
    }

    fn move_piece(&self, start_location: usize, end_location: usize) -> Option<State> {
        //! Moves a piece to a target square provided that the square is valid.
        //! 
        //! Arguments:
        //!     - `start_location`: The current location of the piece in question
        //!     - `end_location`:   The proposed target square.
        
        let mut result: State = self.copy();

        if end_location >= 0 && end_location < 65 {
            let target = self.board[end_location];

            
        }

        return Some(result)
    }

    fn move_b_pawn(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();  // This is basically a placeholder.

        return to_return
    }

    fn move_b_knight(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();

        return to_return
    }

    fn move_b_bishop(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();

        return to_return
    }

    fn move_b_rook(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();

        return to_return
    }

    fn move_b_queen(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();

        return to_return
    }

    fn move_b_king(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();

        return to_return
    }


    fn move_w_pawn(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();

        return to_return
    }

    fn move_w_knight(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();

        return to_return
    }

    fn move_w_bishop(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();

        return to_return
    }

    fn move_w_rook(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();

        return to_return
    }

    fn move_w_queen(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();

        return to_return
    }

    fn move_w_king(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();

        return to_return
    }
}