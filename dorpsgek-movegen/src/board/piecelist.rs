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

use super::index::PieceIndex;
use crate::square::Square;
use std::ops::{Index, IndexMut};

/// A mapping from `PieceIndex` to `Square`.
#[derive(Clone)]
#[repr(transparent)]
pub struct Piecelist([Option<Square>; 32]);

impl Piecelist {
    /// Create a new `Piecelist`.
    pub const fn new() -> Self {
        Self([None; 32])
    }

    /// Add a piece to the board.
    pub fn add_piece(&mut self, piece_index: PieceIndex, square: Square) {
        assert!(
            self[piece_index].is_none(),
            "attempted to add piece to occupied piece index {:?}",
            piece_index
        );
        self[piece_index] = Some(square);
    }

    /// Remove a piece from the board.
    pub fn remove_piece(&mut self, piece_index: PieceIndex, square: Square) {
        match self[piece_index] {
            None => panic!("attempted to remove piece from empty square"),
            Some(square_index) => {
                assert!(
                    square_index == square,
                    "attempted to remove wrong piece from square"
                );
                self[piece_index] = None;
            }
        }
    }

    /// Move a piece in the piecelist.
    pub fn move_piece(&mut self, piece_index: PieceIndex, square: Square) {
        self[piece_index] = Some(square);
    }
}

impl Index<PieceIndex> for Piecelist {
    type Output = Option<Square>;
    fn index(&self, index: PieceIndex) -> &Self::Output {
        &self.0[usize::from(index.into_inner())]
    }
}

impl IndexMut<PieceIndex> for Piecelist {
    fn index_mut(&mut self, index: PieceIndex) -> &mut Self::Output {
        &mut self.0[usize::from(index.into_inner())]
    }
}
