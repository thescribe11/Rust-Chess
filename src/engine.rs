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
            score: 0,
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

    fn move_piece(&self, start_location: usize, end_location: usize) -> Option<State> {
        //! Moves a piece to a target square provided that the square is valid.
        //! 
        //! Arguments:
        //!     - `start_location`: The current location of the piece in question
        //!     - `end_location`:   The proposed target square.
        
        if end_location >= 0 && end_location < 65 {
            let target = self.board[end_location];

            
        }
    }
}