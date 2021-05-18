const ROW: usize = 8;
const BLACK: u8 = 0; const WHITE: u8 = 1;  // Convenience constants for white and black's turns.
const scoresheet: [f32; 6] = [1.0, 3.5, 3.5, 5.25, 10.0, 200.0];


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

    fn get_square(&self, position: usize) -> Option<i8> {
        //! Get if a square is occupied. Returns Option::None if not a valid square.

        if position >=0 && position < 64 {
            return Some(self.board[position])
        } else {
            return None
        }
    }

    fn move_piece(&self, start_location: &usize, end_location: &usize) -> State {
        //! Moves a piece to a target square provided that the square is valid.
        //! 
        //! Arguments:
        //!     - `start_location`: The current location of the piece in question
        //!     - `end_location`:   The proposed target square.
        
        let mut result: State = self.copy();

        let piece = result.board[*start_location];
        result.board[*start_location] = 0;

        if end_location >= &0 && end_location < &65 {
            result.score += scoresheet[result.board[*end_location].abs() as usize];
            result.board[*end_location] = piece;
        }

        result
    }


    fn gen_moves(&self) -> State {
        let mut moves_tree: Vec<State> = Vec::new();

        if self.depth % 2 == 0 {  // It's black's turn. I'm generating moves with the assumption that black goes first, since black is the AI.
            for piece in &self.board {
                
            }
        }
        else {
            for piece in &self.board {

            }
        }

        unimplemented!()
    }


    fn move_b_pawn(&self, position: &usize) -> Vec<State> {
        let mut to_return: Vec<State> = Vec::new();  // This is basically a placeholder.

        if self.get_square(position + ROW) >= 0 {
            to_return.push(self.move_piece(position, &(position + ROW)));
        }

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