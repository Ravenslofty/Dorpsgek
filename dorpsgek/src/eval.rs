use dorpsgek_movegen::{Board, Colour, Piece, Square};

pub struct Eval {
    params: [f64; 5 + 4 + 4],
    pst: [[i32; 64]; 6],
}

impl Eval {
    pub fn new() -> Self {
        let mut s = Self {
            params: [
                1.0, 3.0, 3.5, 5.0, 9.5, -0.1, -0.05, 0.05, 0.1, -0.1, -0.05, 0.05, 0.1,
            ],
            pst: [[0; 64]; 6],
        };
        s.recalculate();
        s
    }

    #[rustfmt::skip]
    pub fn recalculate(&mut self) {
        let piece_values = [self.params[0], self.params[1], self.params[2], self.params[3], self.params[4], 0.0];
        let rank = [self.params[5], self.params[6], self.params[7], self.params[8]];
        let file = [self.params[9], self.params[10], self.params[11], self.params[12]];

        for (piece, piece_value) in piece_values.iter().enumerate() {
            for square in 0..=63 {
                let square_rank = square / 8;
                let square_file = square % 8;
                let mut bonus = 0.0;
                if square_rank <= 3 {
                    bonus += rank[square_rank];
                } else {
                    bonus += rank[7 - square_rank];
                }

                if square_file <= 3 {
                    bonus += file[square_file];
                } else {
                    bonus += file[7 - square_file];
                }

                self.pst[piece][square] = (100.0*(piece_value + bonus)) as i32;
            }
        }
    }

    pub fn eval(&self, board: &Board) -> i32 {
        let mut score = 0;

        for piece in board.pieces() {
            let square = board.square_of_piece(piece);

            if piece.is_white() {
                score += self.piece_square_value(board.piece_from_bit(piece), square);
            } else {
                score -= self.piece_square_value(board.piece_from_bit(piece), square.flip());
            }
        }

        if board.side() == Colour::Black {
            -score
        } else {
            score
        }
    }

    fn piece_square_value(&self, piece: Piece, square: Square) -> i32 {
        match piece {
            Piece::Pawn => self.pst[0][square.into_inner() as usize],
            Piece::Knight => self.pst[1][square.into_inner() as usize],
            Piece::Bishop => self.pst[2][square.into_inner() as usize],
            Piece::Rook => self.pst[3][square.into_inner() as usize],
            Piece::Queen => self.pst[4][square.into_inner() as usize],
            Piece::King => self.pst[5][square.into_inner() as usize],
        }
    }
}
