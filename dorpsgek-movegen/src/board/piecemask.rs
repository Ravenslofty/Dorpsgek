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

use super::{bitlist::Bitlist, index::PieceIndex};
use crate::{colour::Colour, piece::Piece};

#[derive(Clone)]
pub struct Piecemask {
    pbq: Bitlist,
    nbk: Bitlist,
    rqk: Bitlist,
}

impl Piecemask {
    pub const fn new() -> Self {
        Self {
            pbq: Bitlist::new(),
            nbk: Bitlist::new(),
            rqk: Bitlist::new(),
        }
    }

    pub fn empty(&self) -> Bitlist {
        !(self.pbq | self.nbk | self.rqk)
    }

    pub fn occupied(&self) -> Bitlist {
        !self.empty()
    }

    pub fn pawns(&self) -> Bitlist {
        self.pbq & !self.nbk & !self.rqk
    }

    pub fn knights(&self) -> Bitlist {
        !self.pbq & self.nbk & !self.rqk
    }

    pub fn bishops(&self) -> Bitlist {
        self.pbq & self.nbk & !self.rqk
    }

    pub fn rooks(&self) -> Bitlist {
        !self.pbq & !self.nbk & self.rqk
    }

    pub fn queens(&self) -> Bitlist {
        self.pbq & !self.nbk & self.rqk
    }

    pub fn kings(&self) -> Bitlist {
        !self.pbq & self.nbk & self.rqk
    }

    pub fn white(&self) -> Bitlist {
        self.occupied() & Bitlist::white()
    }

    pub fn black(&self) -> Bitlist {
        self.occupied() & Bitlist::black()
    }

    pub fn pieces_of_colour(&self, colour: Colour) -> Bitlist {
        match colour {
            Colour::White => self.white(),
            Colour::Black => self.black(),
        }
    }

    pub fn piece_mask(&self, index: Piece) -> Bitlist {
        match index {
            Piece::Pawn => self.pawns(),
            Piece::Knight => self.knights(),
            Piece::Bishop => self.bishops(),
            Piece::Rook => self.rooks(),
            Piece::Queen => self.queens(),
            Piece::King => self.kings(),
        }
    }

    pub fn colour_mask(&self, index: Colour) -> Bitlist {
        match index {
            Colour::White => self.white(),
            Colour::Black => self.black(),
        }
    }

    pub fn piece(&self, index: PieceIndex) -> Option<Piece> {
        let pbq = self.pbq.contains(index.into());
        let nbk = self.nbk.contains(index.into());
        let rqk = self.rqk.contains(index.into());
        match (pbq, nbk, rqk) {
            (true, false, false) => Some(Piece::Pawn),
            (false, true, false) => Some(Piece::Knight),
            (true, true, false) => Some(Piece::Bishop),
            (false, false, true) => Some(Piece::Rook),
            (true, false, true) => Some(Piece::Queen),
            (false, true, true) => Some(Piece::King),
            (_, _, _) => None,
        }
    }

    /// Add a piece to a Piecemask
    pub fn add_piece(&mut self, piece: Piece, colour: Colour) -> Option<PieceIndex> {
        if let Some(piece_index) = (self.empty() & Bitlist::mask_from_colour(colour)).peek() {
            let yes = Bitlist::from(piece_index);
            let no = Bitlist::new();

            let (pbq, nbk, rqk) = match piece {
                Piece::Pawn => (yes, no, no),
                Piece::Knight => (no, yes, no),
                Piece::Bishop => (yes, yes, no),
                Piece::Rook => (no, no, yes),
                Piece::Queen => (yes, no, yes),
                Piece::King => (no, yes, yes),
            };

            self.pbq |= pbq;
            self.nbk |= nbk;
            self.rqk |= rqk;

            Some(piece_index)
        } else {
            None
        }
    }

    /// Remove a piece from a Piecemask.
    pub fn remove_piece(&mut self, piece_index: PieceIndex) {
        assert!(
            self.occupied().contains(piece_index.into()),
            "attempted to remove invalid piece"
        );
        self.pbq &= !Bitlist::from(piece_index);
        self.nbk &= !Bitlist::from(piece_index);
        self.rqk &= !Bitlist::from(piece_index);
    }
}
