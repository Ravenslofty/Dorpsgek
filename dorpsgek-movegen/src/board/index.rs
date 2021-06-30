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

use crate::{colour::Colour, square::Square};
use std::{
    convert::TryFrom,
    num::NonZeroU8,
    ops::{Index, IndexMut},
};

#[allow(clippy::module_name_repetitions)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct PieceIndex(NonZeroU8);

impl PieceIndex {
    pub const unsafe fn new_unchecked(x: u8) -> Self {
        Self(NonZeroU8::new_unchecked(x + 1))
    }

    pub const fn into_inner(self) -> u8 {
        (self.0.get() - 1) & 31
    }

    pub const fn is_white(self) -> bool {
        self.into_inner() <= 15
    }

    pub const fn is_black(self) -> bool {
        self.into_inner() >= 16
    }
}

impl TryFrom<u8> for PieceIndex {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 31 {
            return Err(());
        }

        // SAFETY: value + 1 is always non-zero.
        Ok(Self(unsafe { NonZeroU8::new_unchecked(value + 1) }))
    }
}

impl From<PieceIndex> for Colour {
    fn from(index: PieceIndex) -> Self {
        if index.is_white() {
            Self::White
        } else {
            Self::Black
        }
    }
}

/// A `Square` -> `PieceIndex` mapping.
#[derive(Clone)]
#[repr(transparent)]
pub struct PieceIndexArray([Option<PieceIndex>; 64]);

impl PieceIndexArray {
    /// Create a new `PieceIndexArray`.
    pub const fn new() -> Self {
        Self([None; 64])
    }

    /// Add a `PieceIndex` to a `Square`. Panics if the square is occupied.
    pub fn add_piece(&mut self, piece_index: PieceIndex, square: Square) {
        assert!(
            self[square].is_none(),
            "attempted to add piece to occupied square"
        );
        self[square] = Some(piece_index);
    }

    /// Remove a `PieceIndex` from a `Square`. Panics if the square is empty or contains a different `PieceIndex`.
    pub fn remove_piece(&mut self, piece_index: PieceIndex, square: Square) {
        match self[square] {
            None => panic!("attempted to remove piece from empty square"),
            Some(square_index) => {
                assert!(
                    square_index == piece_index,
                    "attempted to remove wrong piece from square"
                );
                self[square] = None;
            }
        }
    }

    /// Move a piece from
    pub fn move_piece(
        &mut self,
        piece_index: PieceIndex,
        from_square: Square,
        dest_square: Square,
    ) {
        self[from_square] = None;
        self[dest_square] = Some(piece_index);
    }
}

impl Index<Square> for PieceIndexArray {
    type Output = Option<PieceIndex>;

    fn index(&self, index: Square) -> &Self::Output {
        &self.0[usize::from(index.into_inner())]
    }
}

impl IndexMut<Square> for PieceIndexArray {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self.0[usize::from(index.into_inner())]
    }
}
