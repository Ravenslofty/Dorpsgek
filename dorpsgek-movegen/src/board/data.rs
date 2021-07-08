/*
 *   This file is part of Dorpsgek.
 *
 *   Dorpsgek is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   Dorpsgek is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 *
 *   You should have received a copy of the GNU General Public License
 *   along with Dorpsgek.  If not, see <http://www.gnu.org/licenses/>.
 */

use super::{
    bitlist::{Bitlist, BitlistArray},
    index::{PieceIndex, PieceIndexArray},
    piecelist::Piecelist,
    piecemask::Piecemask,
};
use crate::{
    colour::Colour,
    piece::Piece,
    square::{Direction, Square, Square16x8},
};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct BoardData {
    bitlist: BitlistArray,
    piecelist: Piecelist,
    index: PieceIndexArray,
    piecemask: Piecemask,
}

impl BoardData {
    /// Create a new board.
    pub const fn new() -> Self {
        Self {
            bitlist: BitlistArray::new(),
            piecelist: Piecelist::new(),
            index: PieceIndexArray::new(),
            piecemask: Piecemask::new(),
        }
    }

    /// Return the piece index on a square, if any.
    pub fn piece_index(&self, square: Square) -> Option<PieceIndex> {
        self.index[square]
    }

    /// Return the attacks to a square by a colour.
    pub fn attacks_to(&self, square: Square, colour: Colour) -> Bitlist {
        self.bitlist[square] & Bitlist::mask_from_colour(colour)
    }

    /// Return the square a piece resides on.
    pub fn square_of_piece(&self, bit: PieceIndex) -> Square {
        self.piecelist.get(bit)
    }

    /// True if the square has a piece on it.
    pub fn has_piece(&self, square: Square) -> bool {
        self.index[square].is_some()
    }

    /// Return a bitlist of all pawns.
    pub const fn pawns(&self) -> Bitlist {
        self.piecemask.pawns()
    }

    /// Return a bitlist of all knights.
    pub const fn knights(&self) -> Bitlist {
        self.piecemask.knights()
    }

    /// Return a bitlist of all bishops.
    pub const fn bishops(&self) -> Bitlist {
        self.piecemask.bishops()
    }

    /// Return a bitlist of all rooks.
    pub const fn rooks(&self) -> Bitlist {
        self.piecemask.rooks()
    }

    /// Return a bitlist of all queens.
    pub const fn queens(&self) -> Bitlist {
        self.piecemask.queens()
    }

    /// Return a bitlist of all kings.
    pub const fn kings(&self) -> Bitlist {
        self.piecemask.kings()
    }

    /// Return a bitlist of all pieces.
    pub const fn pieces(&self) -> Bitlist {
        self.piecemask.occupied()
    }

    /// Return a bitlist of all pieces of a given colour.
    pub const fn pieces_of_colour(&self, colour: Colour) -> Bitlist {
        self.piecemask.pieces_of_colour(colour)
    }

    /// Given a piece index, return its piece type.
    pub fn piece_from_bit(&self, bit: PieceIndex) -> Piece {
        self.piecemask
            .piece(bit)
            .expect("piece index corresponds to invalid piece")
    }

    /// Given a square, return the piece type of it, if any.
    pub fn piece_from_square(&self, square: Square) -> Option<Piece> {
        self.piecemask.piece(self.index[square]?)
    }

    /// Given a square, return the colour of the piece on it, if any.
    pub fn colour_from_square(&self, square: Square) -> Option<Colour> {
        Some(Colour::from(self.index[square]?))
    }

    /// Add a `Piece` to a `Square`.
    pub fn add_piece(&mut self, piece: Piece, colour: Colour, square: Square, update: bool) {
        let piece_index = self.piecemask.add_piece(piece, colour);
        self.piecelist.add_piece(piece_index, square);
        self.index.add_piece(piece_index, square);

        if update {
            self.update_attacks(square, piece_index, piece, true);
            self.update_sliders(square, false);
        }
    }

    /// Remove a piece from a square.
    pub fn remove_piece(&mut self, piece_index: PieceIndex, update: bool) {
        let square = self.piecelist.get(piece_index);
        let piece = self.piece_from_bit(piece_index);
        self.piecemask.remove_piece(piece_index);
        self.piecelist.remove_piece(piece_index, square);
        self.index.remove_piece(piece_index, square);

        if update {
            self.update_attacks(square, piece_index, piece, false);
            self.update_sliders(square, true);
        }
    }

    /// Move a piece from a square to another square.
    pub fn move_piece(&mut self, from_square: Square, to_square: Square) {
        let piece_index =
            self.index[from_square].expect("attempted to move piece from empty square");
        let piece = self.piece_from_bit(piece_index);

        self.update_attacks(from_square, piece_index, piece, false);
        self.update_sliders(from_square, true);

        self.piecelist.move_piece(piece_index, to_square);
        self.index.move_piece(piece_index, from_square, to_square);

        self.update_attacks(to_square, piece_index, piece, true);
        self.update_sliders(to_square, false);

        debug_assert!(
            !self.bitlist[to_square].contains(piece_index.into()),
            "piece on {} cannot attack itself",
            to_square
        );
    }

    /// Rebuild the attack set for the board.
    pub fn rebuild_attacks(&mut self) {
        for square in 0_u8..64 {
            // SAFETY: index is always in bounds.
            let index = unsafe { Square::from_u8_unchecked(square) };
            self.bitlist.clear(index);
        }

        for square in 0_u8..64 {
            // SAFETY: square is always in bounds.
            let square = unsafe { Square::from_u8_unchecked(square) };
            if let Some(bit) = self.index[square] {
                let piece = self.piece_from_bit(bit);
                self.update_attacks(square, bit, piece, true);
            }
        }
    }

    /// Add or remove attacks for a square.
    #[allow(clippy::too_many_lines)]
    fn update_attacks(
        &mut self,
        square: Square,
        bit: PieceIndex,
        piece: Piece,
        add: bool,
    ) {
        static DIRECTIONS: [Option<Direction>; 148] = [
            // Knight (normal) @ 0
            Some(Direction::NorthNorthEast),
            Some(Direction::EastNorthEast),
            Some(Direction::EastSouthEast),
            Some(Direction::SouthSouthEast),
            Some(Direction::SouthSouthWest),
            Some(Direction::WestSouthWest),
            Some(Direction::WestNorthWest),
            Some(Direction::NorthNorthWest),
            None,
            // Knight (left edge) @ 9
            Some(Direction::NorthNorthEast),
            Some(Direction::EastNorthEast),
            Some(Direction::EastSouthEast),
            Some(Direction::SouthSouthEast),
            None,
            // Knight (right edge) @ 14
            Some(Direction::SouthSouthWest),
            Some(Direction::WestSouthWest),
            Some(Direction::WestNorthWest),
            Some(Direction::NorthNorthWest),
            None,
            // Knight (top edge) @ 19
            Some(Direction::EastSouthEast),
            Some(Direction::SouthSouthEast),
            Some(Direction::SouthSouthWest),
            Some(Direction::WestSouthWest),
            None,
            // Knight (bottom edge) @ 24
            Some(Direction::NorthNorthEast),
            Some(Direction::EastNorthEast),
            Some(Direction::WestNorthWest),
            Some(Direction::NorthNorthWest),
            None,
            // Knight (bottom left corner) @ 29
            Some(Direction::NorthNorthEast),
            Some(Direction::EastNorthEast),
            None,
            // Knight (bottom right corner) @ 32
            Some(Direction::WestNorthWest),
            Some(Direction::NorthNorthWest),
            None,
            // Knight (top left corner) @ 35
            Some(Direction::SouthSouthEast),
            Some(Direction::EastSouthEast),
            None,
            // Knight (top right corner) @ 38
            Some(Direction::WestSouthWest),
            Some(Direction::SouthSouthWest),
            None,
            // Bishop (normal) @ 0
            Some(Direction::NorthEast),
            Some(Direction::SouthEast),
            Some(Direction::SouthWest),
            Some(Direction::NorthWest),
            None,
            // Bishop (left edge) @ 5
            Some(Direction::NorthEast),
            Some(Direction::SouthEast),
            None,
            // Bishop (right edge) @ 8
            Some(Direction::NorthWest),
            Some(Direction::SouthWest),
            None,
            // Bishop (top edge) @ 11
            Some(Direction::SouthEast),
            Some(Direction::SouthWest),
            None,
            // Bishop (bottom edge) @ 14
            Some(Direction::NorthEast),
            Some(Direction::NorthWest),
            None,
            // Bishop (bottom left corner) @ 17
            Some(Direction::NorthEast),
            None,
            // Bishop (bottom right corner) @ 19
            Some(Direction::NorthWest),
            None,
            // Bishop (top left corner) @ 21
            Some(Direction::SouthEast),
            None,
            // Bishop (top right corner) @ 23
            Some(Direction::SouthWest),
            None,
            // Rook (normal) @ 0
            Some(Direction::North),
            Some(Direction::East),
            Some(Direction::South),
            Some(Direction::West),
            None,
            // Rook (left edge) @ 5
            Some(Direction::North),
            Some(Direction::East),
            Some(Direction::South),
            None,
            // Rook (right edge) @ 9
            Some(Direction::North),
            Some(Direction::West),
            Some(Direction::South),
            None,
            // Rook (top edge) @ 13
            Some(Direction::East),
            Some(Direction::West),
            Some(Direction::South),
            None,
            // Rook (bottom edge) @ 17
            Some(Direction::North),
            Some(Direction::East),
            Some(Direction::West),
            None,
            // Rook (bottom left corner) @ 21
            Some(Direction::North),
            Some(Direction::East),
            None,
            // Rook (bottom right corner) @ 24
            Some(Direction::North),
            Some(Direction::West),
            None,
            // Rook (top left corner) @ 27
            Some(Direction::South),
            Some(Direction::East),
            None,
            // Rook (top right corner) @ 30
            Some(Direction::South),
            Some(Direction::West),
            None,
            // Queen + King (normal) @ 0
            Some(Direction::North),
            Some(Direction::NorthEast),
            Some(Direction::East),
            Some(Direction::SouthEast),
            Some(Direction::South),
            Some(Direction::SouthWest),
            Some(Direction::West),
            Some(Direction::NorthWest),
            None,
            // Queen + King (left edge) @ 9
            Some(Direction::North),
            Some(Direction::NorthEast),
            Some(Direction::East),
            Some(Direction::SouthEast),
            Some(Direction::South),
            None,
            // Queen + King (right edge) @ 15
            Some(Direction::North),
            Some(Direction::South),
            Some(Direction::SouthWest),
            Some(Direction::West),
            Some(Direction::NorthWest),
            None,
            // Queen + King (top edge) @ 21
            Some(Direction::East),
            Some(Direction::SouthEast),
            Some(Direction::South),
            Some(Direction::SouthWest),
            Some(Direction::West),
            None,
            // Queen + King (bottom edge) @ 27
            Some(Direction::North),
            Some(Direction::NorthEast),
            Some(Direction::East),
            Some(Direction::West),
            Some(Direction::NorthWest),
            None,
            // Queen + King (bottom left corner) @ 33
            Some(Direction::North),
            Some(Direction::NorthEast),
            Some(Direction::East),
            None,
            // Queen + King (bottom right corner) @ 37
            Some(Direction::North),
            Some(Direction::NorthWest),
            Some(Direction::West),
            None,
            // Queen + King (top left corner) @ 41
            Some(Direction::South),
            Some(Direction::SouthEast),
            Some(Direction::East),
            None,
            // Queen + King (top right corner) @ 45
            Some(Direction::South),
            Some(Direction::SouthWest),
            Some(Direction::West),
            None
        ];

        const KNIGHT_ATTACKS: usize = 41;
        const BISHOP_ATTACKS: usize = 25;
        const ROOK_ATTACKS: usize = 33;

        static BASE: [usize; 6] = [
            0,  // Pawn
            0,  // Knight
            KNIGHT_ATTACKS,  // Bishop
            KNIGHT_ATTACKS + BISHOP_ATTACKS, // Rook
            KNIGHT_ATTACKS + BISHOP_ATTACKS + ROOK_ATTACKS, // Queen
            KNIGHT_ATTACKS + BISHOP_ATTACKS + ROOK_ATTACKS, // King
        ];

        #[rustfmt::skip]
        static OFFSET: [[usize; 64]; 6] = [
            // Pawn
            [0; 64],
            // Knight,
            [
                29, 24, 24, 24, 24, 24, 24, 32,
                 9,  0,  0,  0,  0,  0,  0, 14,
                 9,  0,  0,  0,  0,  0,  0, 14,
                 9,  0,  0,  0,  0,  0,  0, 14,
                 9,  0,  0,  0,  0,  0,  0, 14,
                 9,  0,  0,  0,  0,  0,  0, 14,
                 9,  0,  0,  0,  0,  0,  0, 14,
                35, 19, 19, 19, 19, 19, 19, 38,
            ],
            // Bishop,
            [
                17, 14, 14, 14, 14, 14, 14, 19,
                 5,  0,  0,  0,  0,  0,  0,  8,
                 5,  0,  0,  0,  0,  0,  0,  8,
                 5,  0,  0,  0,  0,  0,  0,  8,
                 5,  0,  0,  0,  0,  0,  0,  8,
                 5,  0,  0,  0,  0,  0,  0,  8,
                 5,  0,  0,  0,  0,  0,  0,  8,
                21, 11, 11, 11, 11, 11, 11, 23,
            ],
            // Rook,
            [
                21, 17, 17, 17, 17, 17, 17, 24,
                 5,  0,  0,  0,  0,  0,  0,  9,
                 5,  0,  0,  0,  0,  0,  0,  9,
                 5,  0,  0,  0,  0,  0,  0,  9,
                 5,  0,  0,  0,  0,  0,  0,  9,
                 5,  0,  0,  0,  0,  0,  0,  9,
                 5,  0,  0,  0,  0,  0,  0,  9,
                27, 13, 13, 13, 13, 13, 13, 30,
            ],
            // Queen,
            [
                33, 27, 27, 27, 27, 27, 27, 37,
                 9,  0,  0,  0,  0,  0,  0, 15,
                 9,  0,  0,  0,  0,  0,  0, 15,
                 9,  0,  0,  0,  0,  0,  0, 15,
                 9,  0,  0,  0,  0,  0,  0, 15,
                 9,  0,  0,  0,  0,  0,  0, 15,
                 9,  0,  0,  0,  0,  0,  0, 15,
                41, 21, 21, 21, 21, 21, 21, 45,
            ],
            // King,
            [
                33, 27, 27, 27, 27, 27, 27, 37,
                 9,  0,  0,  0,  0,  0,  0, 15,
                 9,  0,  0,  0,  0,  0,  0, 15,
                 9,  0,  0,  0,  0,  0,  0, 15,
                 9,  0,  0,  0,  0,  0,  0, 15,
                 9,  0,  0,  0,  0,  0,  0, 15,
                 9,  0,  0,  0,  0,  0,  0, 15,
                41, 21, 21, 21, 21, 21, 21, 45,
            ],
        ];

        let update = |bitlist: &mut BitlistArray, dest: Square| {
            if add {
                debug_assert!(dest != square);
                bitlist.add_piece(dest, bit);
            } else {
                bitlist.remove_piece(dest, bit);
            }
        };

        if piece == Piece::Pawn {
            square.pawn_attacks(Colour::from(bit)).for_each(|dest| update(&mut self.bitlist, dest));
        } else if matches!(piece, Piece::Knight | Piece::King) {
            let mut offset = BASE[piece as usize] + OFFSET[piece as usize][square.into_inner() as usize];
            while let Some(direction) = unsafe { DIRECTIONS.get_unchecked(offset) } {
                if let Some(dest) = square.travel(*direction) {
                    update(&mut self.bitlist, dest);
                }
                offset += 1;
            }
        } else {
            let mut offset = BASE[piece as usize] + OFFSET[piece as usize][square.into_inner() as usize];
            while let Some(direction) = unsafe { DIRECTIONS.get_unchecked(offset) } {
                for dest in Square16x8::from_square(square).ray_attacks(*direction) {
                    update(&mut self.bitlist, dest);
                    if self.index[dest].is_some() {
                        break;
                    }
                }
                offset += 1;
            }
        }
    }

    /// Extend or remove slider attacks to a square.
    fn update_sliders(&mut self, square: Square, add: bool) {
        let sliders = self.bitlist[square]
            & (self.piecemask.bishops() | self.piecemask.rooks() | self.piecemask.queens());

        let square = Square16x8::from_square(square);
        for piece in sliders {
            let attacker = Square16x8::from_square(self.piecelist.get(piece));
            if let Some(direction) = attacker.direction(square) {
                for dest in square.ray_attacks(direction) {
                    if add {
                        self.bitlist.add_piece(dest, piece);
                    } else {
                        self.bitlist.remove_piece(dest, piece);
                    }

                    if self.index[dest].is_some() {
                        break;
                    }
                }
            } /* else {
                let attacker = attacker.to_square().unwrap();
                let square = square.to_square().unwrap();
                panic!(
                    "no direction between {:?} {} and {:?} {}",
                    self.piece_from_square(attacker),
                    attacker,
                    self.piece_from_square(square),
                    square
                );
            } */
        }
    }
}
