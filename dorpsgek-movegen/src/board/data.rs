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
    square::{Direction, Square},
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

    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn bitlists(&self) -> BitlistArray {
        self.bitlist.clone()
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

    /// Return a bitlist of all pieces of a given colour.
    pub const fn pieces_of_colour(&self, colour: Colour) -> Bitlist {
        self.piecemask.pieces_of_colour(colour)
    }

    /// Given a piece index, return its piece type.
    pub fn piece_from_bit(&self, bit: PieceIndex) -> Piece {
        self.piecemask.piece(bit).expect("piece index corresponds to invalid piece")
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
            self.update_attacks(square, piece_index, piece, true, None);
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
            self.update_attacks(square, piece_index, piece, false, None);
            self.update_sliders(square, true);
        }
    }

    /// Move a piece from a square to another square.
    pub fn move_piece(&mut self, from_square: Square, to_square: Square, update: bool, quiet: bool) {
        let piece_index =
            self.index[from_square].expect("attempted to move piece from empty square");
        let piece = self.piece_from_bit(piece_index);
        let slide_dir = from_square.direction(to_square).and_then(|dir| if matches!(piece, Piece::Bishop | Piece::Rook | Piece::Queen) && quiet { Some(dir) } else { None } );

        if update {
            self.update_attacks(from_square, piece_index, piece, false, slide_dir);
            self.update_sliders(from_square, true);
            if slide_dir.is_some() && quiet {
                self.bitlist.add_piece(from_square, piece_index);
            }
        }

        self.piecelist.move_piece(piece_index, to_square);
        self.index.move_piece(piece_index, from_square, to_square);

        if update {
            self.update_attacks(to_square, piece_index, piece, true, slide_dir);
            self.update_sliders(to_square, false);
            if slide_dir.is_some() && quiet {
                self.bitlist.remove_piece(to_square, piece_index);
            }
        }
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
                self.update_attacks(square, bit, piece, true, None);
            }
        }
    }

    /// Add or remove attacks for a square.
    fn update_attacks(&mut self, square: Square, bit: PieceIndex, piece: Piece, add: bool, skip_dir: Option<Direction>) {
        let update = |b: &mut BitlistArray, dest: Square| {
            if add {
                b.add_piece(dest, bit);
            } else {
                b.remove_piece(dest, bit);
            }
        };

        let mut slide = |dir: Direction, square: Square| {
            if let Some(slide_dir) = skip_dir {
                if slide_dir == dir || slide_dir == dir.opposite() {
                    return;
                }
            }

            for dest in square.ray_attacks(dir) {
                update(&mut self.bitlist, dest);
                if self.index[dest].is_some() {
                    break;
                }
            }
        };

        match piece {
            Piece::Pawn => square
                .pawn_attacks(Colour::from(bit))
                .for_each(|dest| update(&mut self.bitlist, dest)),
            Piece::Knight => square
                .knight_attacks()
                .for_each(|dest| update(&mut self.bitlist, dest)),
            Piece::King => square
                .king_attacks()
                .for_each(|dest| update(&mut self.bitlist, dest)),
            Piece::Bishop => {
                slide(Direction::NorthEast, square);
                slide(Direction::SouthEast, square);
                slide(Direction::SouthWest, square);
                slide(Direction::NorthWest, square);
            },
            Piece::Rook => {
                slide(Direction::North, square);
                slide(Direction::East, square);
                slide(Direction::South, square);
                slide(Direction::West, square);
            },
            Piece::Queen => {
                slide(Direction::North, square);
                slide(Direction::East, square);
                slide(Direction::South, square);
                slide(Direction::West, square);
                slide(Direction::NorthEast, square);
                slide(Direction::SouthEast, square);
                slide(Direction::SouthWest, square);
                slide(Direction::NorthWest, square);
            }
        };
    }

    /// Extend or remove slider attacks to a square.
    fn update_sliders(&mut self, square: Square, add: bool) {
        let sliders: Bitlist = self.bitlist[square]
            & (self.piecemask.bishops() | self.piecemask.rooks() | self.piecemask.queens());

        for piece in sliders {
            let attacker = self.piecelist.get(piece);
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
            }
        }
    }
}
