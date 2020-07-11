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

use crate::{
    chessmove::{Move, MoveType},
    colour::Colour,
    piece::Piece,
    square::{Direction, File, Rank, Square},
};
use std::{
    convert::{TryFrom, TryInto},
    ffi::CString,
    fmt::Display,
};

use tinyvec::ArrayVec;

use bitlist::Bitlist;
use data::BoardData;

mod bitlist;
mod data;
mod index;
mod piecelist;
mod piecemask;

/// A chess position.
#[derive(Clone)]
pub struct Board {
    /// The chess board representation.
    data: data::BoardData,
    /// The side to move.
    side: Colour,
    /// Castling rights, if any.
    castle: u8,
    /// En-passant square, if any.
    ep: Option<Square>,
}

impl Default for Board {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0_u8..64_u8 {
            let j = i ^ 56_u8;

            if let (Some(piece), Some(colour)) = (
                self.data
                    .piece_from_square(j.try_into().expect("square somehow out of bounds")),
                self.data
                    .colour_from_square(j.try_into().expect("square somehow out of bounds")),
            ) {
                let c = match piece {
                    Piece::Pawn => 'P',
                    Piece::Knight => 'N',
                    Piece::Bishop => 'B',
                    Piece::Rook => 'R',
                    Piece::Queen => 'Q',
                    Piece::King => 'K',
                };

                let c = match colour {
                    Colour::White => c.to_ascii_uppercase(),
                    Colour::Black => c.to_ascii_lowercase(),
                };

                write!(f, "{}", c)?;
            } else {
                write!(f, ".")?;
            }

            if j & 7 == 7 {
                writeln!(f)?;
            }
        }
        if self.side == Colour::White {
            writeln!(f, "White to move.")?;
        } else {
            writeln!(f, "Black to move.")?;
        }
        if self.castle & 1 != 0 {
            write!(f, "K")?;
        }
        if self.castle & 2 != 0 {
            write!(f, "Q")?;
        }
        if self.castle & 4 != 0 {
            write!(f, "k")?;
        }
        if self.castle & 8 != 0 {
            write!(f, "q")?;
        }
        writeln!(f)?;
        if let Some(ep) = self.ep {
            writeln!(f, "{}", ep)?;
        } else {
            writeln!(f, "-")?;
        }

        Ok(())
    }
}

impl Board {
    /// Create a new empty board.
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            side: Colour::White,
            castle: 0,
            ep: None,
            data: BoardData::new(),
        }
    }

    /// Check if this board is illegal by seeing if the enemy king is attacked by friendly pieces.
    /// If it is, it implies the move the enemy made left them in check, which is illegal.
    #[must_use]
    #[inline]
    pub fn illegal(&self) -> bool {
        if let Some(king_index) = (self.data.kings() & self.data.pieces_of_colour(!self.side)).peek() {
            let king_square = self.data.square_of_piece(king_index);
            !self.data.attacks_to(king_square, self.side).empty()
        } else {
            // Not having a king is very definitely illegal.
            false
        }
    }

    /// Parse a position in Forsyth-Edwards Notation into a board.
    #[must_use]
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn from_fen(fen: &str) -> Option<Self> {
        let fen = CString::new(fen).expect("FEN is not ASCII");
        let fen = fen.as_bytes();
        Self::from_fen_bytes(fen)
    }

    /// Parse a position in Forsyth-Edwards Notation into a board.
    #[must_use]
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn from_fen_bytes(fen: &[u8]) -> Option<Self> {
        let mut b = Self::new();

        let mut idx = 0_usize;
        let mut c = fen[idx];

        for rank in (0..=7).rev() {
            let mut file = 0;
            while file <= 7 {
                if c >= b'1' && c <= b'8' {
                    let length = c - b'0';
                    let mut i = 0;
                    while i < length {
                        file += 1;
                        i += 1
                    }
                } else {
                    let piece = match c.to_ascii_lowercase() {
                        b'k' => Piece::King,
                        b'q' => Piece::Queen,
                        b'r' => Piece::Rook,
                        b'b' => Piece::Bishop,
                        b'n' => Piece::Knight,
                        b'p' => Piece::Pawn,
                        _ => return None,
                    };

                    let colour = if c.is_ascii_uppercase() {
                        Colour::White
                    } else {
                        Colour::Black
                    };

                    let square =
                        Square::from_rank_file(rank.try_into().unwrap(), file.try_into().unwrap());

                    b.data.add_piece(piece, colour, square, false);

                    file += 1
                }
                idx += 1;
                c = fen[idx]
            }
            if rank > 0 {
                idx += 1;
                c = fen[idx]
            }
        }
        idx += 1;
        c = fen[idx];
        if c == b'w' {
            b.side = Colour::White;
        } else if c == b'b' {
            b.side = Colour::Black;
        } else {
            return None;
        }
        idx += 2;
        c = fen[idx];
        b.castle = 0;
        if c == b'-' {
            idx += 1;
        } else {
            if c == b'K' {
                b.castle |= 1;
                idx += 1;
                c = fen[idx]
            }
            if c == b'Q' {
                b.castle |= 2;
                idx += 1;
                c = fen[idx]
            }
            if c == b'k' {
                b.castle |= 4;
                idx += 1;
                c = fen[idx]
            }
            if c == b'q' {
                b.castle |= 8;
                idx += 1;
            }
        }
        idx += 1;
        c = fen[idx];
        if c == b'-' {
            b.ep = None;
        } else {
            let file = File::try_from(c - b'a').unwrap();
            idx += 1;
            c = fen[idx];
            let rank = Rank::try_from(c - b'1').unwrap();
            b.ep = Some(Square::from_rank_file(rank, file));
        }

        b.data.rebuild_attacks();

        Some(b)
    }

    /// Make a move on the board.
    #[inline]
    #[must_use]
    pub fn make(&self, m: Move) -> Self {
        let mut b = self.clone();
        match m.kind {
            MoveType::Normal => {
                b.data.move_piece(m.from, m.dest, true);
                b.ep = None;
            }
            MoveType::DoublePush => {
                b.data.move_piece(m.from, m.dest, true);
                b.ep = m.from.relative_north(b.side);
            }
            MoveType::Capture => {
                let piece_index = b
                    .data
                    .piece_index(m.dest)
                    .expect("attempted to capture an empty square");
                b.data.remove_piece(piece_index, true);
                b.data.move_piece(m.from, m.dest, true);
                b.ep = None;
            }
            MoveType::Castle
            | MoveType::EnPassant
            | MoveType::Promotion
            | MoveType::CapturePromotion => todo!(),
        }

        b.side = !b.side;
        b
    }

    /// Find pinned pieces and handle them specially.
    fn generate_pinned_pieces(&self, v: &mut ArrayVec<[Move; 256]>) -> Bitlist {
        let mut pinned = Bitlist::new();

        let diagonal = self.data.bishops() | self.data.queens();
        let orthogonal = self.data.rooks() | self.data.queens();
        let king_index = (self.data.kings() & Bitlist::mask_from_colour(self.side))
            .peek()
            .unwrap();
        let king_square = self.data.square_of_piece(king_index);

        let count_blockers = |pinner_king_dir, pinner_square| {
            let mut blocker_count = 0;
            for square in king_square.ray_attacks(pinner_king_dir) {
                if self.data.has_piece(square) {
                    // We found the pinner without finding multiple blockers; the blocker is pinned.
                    if square == pinner_square {
                        break;
                    }

                    blocker_count += 1;
                }
            }
            blocker_count
        };

        // If a friendly piece has attacks from an enemy slider that lines up with the king, and no other blockers, consider it pinned.
        'find_blockers: for bit in self.data.pieces_of_colour(self.side) & !self.data.kings() {
            let blocker_square = self.data.square_of_piece(bit);
            let attacks = self.data.attacks_to(blocker_square, !self.side);

            // If the king and blocker do not share a ray, this piece cannot possibly be pinned.
            let blocker_king_dir = match king_square.direction(blocker_square) {
                Some(dir) => dir,
                None => continue,
            };

            for attacker in attacks & diagonal {
                let pinner_square = self.data.square_of_piece(attacker);

                // If the king and pinner do not share a ray, this piece cannot possibly pin the blocker to the king.
                let pinner_king_dir = match king_square.direction(pinner_square) {
                    Some(dir) => dir,
                    None => continue,
                };

                // If the king to pinner ray and king to blocker ray are different, this piece cannot possibly pin the blocker to the king.
                if blocker_king_dir != pinner_king_dir {
                    continue;
                }

                // If this is a queen, it might be an orthogonal ray; we don't care about it here.
                if !pinner_king_dir.diagonal() {
                    continue;
                }

                // Look for blocking pieces
                // For diagonals, simply having one extra blocker is enough.
                if count_blockers(pinner_king_dir, pinner_square) > 1 {
                    continue;
                }

                // This piece is pinned.
                match self.data.piece_from_bit(bit) {
                    Piece::Pawn => self.generate_pawn(v, blocker_square, Some(pinner_king_dir)),
                    Piece::Knight | Piece::Rook | Piece::King => {} // Knights cannot move when pinned; Rooks cannot move when pinned diagonally; Kings should not be pinned.
                    Piece::Bishop | Piece::Queen => {
                        // Bishops/Queens can slide along the king to blocker ray.
                        for dest in king_square.ray_attacks(pinner_king_dir) {
                            if dest == blocker_square {
                                continue;
                            }
                            if dest == pinner_square {
                                v.push(Move::new(blocker_square, dest, MoveType::Capture, None));
                                break;
                            }
                            v.push(Move::new(blocker_square, dest, MoveType::Normal, None));
                        }
                    }
                }

                pinned |= Bitlist::from(bit);

                continue 'find_blockers;
            }

            for attacker in attacks & orthogonal {
                let pinner_square = self.data.square_of_piece(attacker);

                // If the king and pinner do not share a ray, this piece cannot possibly pin the blocker to the king.
                let pinner_king_dir = match king_square.direction(pinner_square) {
                    Some(dir) => dir,
                    None => continue,
                };

                // If the king to pinner ray and king to blocker ray are different, this piece cannot possibly pin the blocker to the king.
                if blocker_king_dir != pinner_king_dir {
                    continue;
                }

                // If this is a queen, it might be a diagonal ray; we don't care about it here.
                if pinner_king_dir.diagonal() {
                    continue;
                }

                // Look for blocking pieces
                let blocker_count = count_blockers(pinner_king_dir, pinner_square);

                // For pieces, simply having one extra blocker is enough.
                if self.data.piece_from_bit(bit) != Piece::Pawn && blocker_count > 1 {
                    continue;
                }

                // This piece is pinned.
                match self.data.piece_from_bit(bit) {
                    Piece::Pawn => self.generate_pawn(v, blocker_square, Some(pinner_king_dir)),
                    Piece::Knight | Piece::Bishop | Piece::King => {} // Knights cannot move when pinned; Rooks cannot move when pinned diagonally; Kings should not be pinned.
                    Piece::Rook | Piece::Queen => {
                        // Bishops/Queens can slide along the king to blocker ray.
                        for dest in king_square.ray_attacks(pinner_king_dir) {
                            if dest == blocker_square {
                                continue;
                            }
                            if dest == pinner_square {
                                v.push(Move::new(blocker_square, dest, MoveType::Capture, None));
                                break;
                            }
                            v.push(Move::new(blocker_square, dest, MoveType::Normal, None));
                        }
                    }
                }

                pinned |= Bitlist::from(bit);

                continue 'find_blockers;
            }
        }

        pinned
    }

    /// Generate pawn-specific moves.
    fn generate_pawn(&self, v: &mut ArrayVec<[Move; 256]>, from: Square, dir: Option<Direction>) {
        let push = |
            v: &mut ArrayVec<[Move; 256]>,
            from: Square,
            dest: Square,
            kind: MoveType,
            prom: Option<Piece>,
            dir: Option<Direction>,
        | {
            if let Some(dir) = dir {
                if let Some(move_dir) = from.direction(dest) {
                    if dir != move_dir && dir != move_dir.opposite() {
                        return;
                    }
                }
            }
            v.push(Move::new(from, dest, kind, prom));
        };

        let add_captures = |dest, v: &mut ArrayVec<[Move; 256]>| {
            if let Some(colour) = self.data.colour_from_square(dest) {
                if colour != self.side {
                    if Rank::from(dest).is_relative_eighth(self.side) {
                        push(
                            v,
                            from,
                            dest,
                            MoveType::CapturePromotion,
                            Some(Piece::Queen),
                            dir,
                        );
                        push(
                            v,
                            from,
                            dest,
                            MoveType::CapturePromotion,
                            Some(Piece::Rook),
                            dir,
                        );
                        push(
                            v,
                            from,
                            dest,
                            MoveType::CapturePromotion,
                            Some(Piece::Bishop),
                            dir,
                        );
                        push(
                            v,
                            from,
                            dest,
                            MoveType::CapturePromotion,
                            Some(Piece::Knight),
                            dir,
                        );
                    } else {
                        push(v, from, dest, MoveType::Capture, None, dir);
                    }
                }
            }
        };

        let north = from.relative_north(self.side);
        if let Some(dest) = north {
            // Pawn single pushes.
            if !self.data.has_piece(dest) {
                if Rank::from(dest).is_relative_eighth(self.side) {
                    push(v, from, dest, MoveType::Promotion, Some(Piece::Queen), dir);
                    push(v, from, dest, MoveType::Promotion, Some(Piece::Rook), dir);
                    push(v, from, dest, MoveType::Promotion, Some(Piece::Bishop), dir);
                    push(v, from, dest, MoveType::Promotion, Some(Piece::Knight), dir);
                } else {
                    push(v, from, dest, MoveType::Normal, None, dir);
                }

                // Pawn double pushes.
                let north2 = dest.relative_north(self.side);
                if let Some(dest) = north2 {
                    if Rank::from(dest).is_relative_fourth(self.side) && !self.data.has_piece(dest)
                    {
                        push(v, from, dest, MoveType::DoublePush, None, dir);
                    }
                }
            }

            let add_en_passant = |dest: Square, v: &mut ArrayVec<[Move; 256]>| {
                if let Some(ep) = self.ep {
                    if let Some(Piece::Pawn) = self.data.piece_from_square(ep) {
                        if ep == dest {
                            push(v, from, dest, MoveType::EnPassant, None, dir);
                        }
                    }
                }
            };

            // Pawn captures.
            if let Some(dest) = dest.east() {
                add_captures(dest, v);
                add_en_passant(dest, v);
            }

            if let Some(dest) = dest.west() {
                add_captures(dest, v);
                add_en_passant(dest, v);
            }
        }
    }

    /// Generate pawn-specific moves.
    fn generate_pawns(&self, v: &mut ArrayVec<[Move; 256]>, pinned: Bitlist) {
        for pawn in self.data.pawns() & Bitlist::mask_from_colour(self.side) & !pinned {
            let from = self.data.square_of_piece(pawn);
            self.generate_pawn(v, from, None);
        }
    }

    /// Generate king-specific moves.
    fn generate_king(&self, v: &mut ArrayVec<[Move; 256]>) {
        if let Some(piece_index) = (self.data.kings() & Bitlist::mask_from_colour(self.side)).peek()
        {
            let square = self.data.square_of_piece(piece_index);

            for dest in square.king_attacks() {
                let mut kind = MoveType::Normal;

                if let Some(colour) = self.data.colour_from_square(dest) {
                    if colour == self.side {
                        // Forbid own-colour captures.
                        continue;
                    } else {
                        kind = MoveType::Capture;
                    }
                }

                // It's illegal for kings to move to attacked squares; prune those out.
                if !self.data.attacks_to(dest, !self.side).empty() {
                    continue;
                }

                v.push(Move::new(square, dest, kind, None));
            }
        }
    }

    /// Generate moves when in check by a single piece.
    fn generate_single_check(&self, v: &mut ArrayVec<[Move; 256]>) {
        #[allow(clippy::option_unwrap_used)]
        let king_index = (self.data.kings() & Bitlist::mask_from_colour(self.side))
            .peek()
            .unwrap();
        let king_square = self.data.square_of_piece(king_index);
        let attacker_bit = self.data.attacks_to(king_square, !self.side);
        let attacker_index = attacker_bit.peek().unwrap();
        let attacker_piece = self.data.piece_from_bit(attacker_index);
        let attacker_square = self.data.square_of_piece(attacker_index);

        let add_pawn_block = |v: &mut ArrayVec<[Move; 256]>, from, dest, kind| {
            if let Some(colour) = self.data.colour_from_square(from) {
                if colour == self.side {
                    v.push(Move::new(from, dest, kind, None));
                }
            }
        };

        let add_pawn_blocks = |v: &mut ArrayVec<[Move; 256]>, dest: Square| {
            if let Some(from) = dest.relative_south(self.side) {
                match self.data.piece_from_square(from) {
                    Some(Piece::Pawn) => {
                        add_pawn_block(v, from, dest, MoveType::Normal);
                    }
                    None if Rank::from(dest).is_relative_fourth(self.side) => {
                        if let Some(from) = from.relative_south(self.side) {
                            if let Some(Piece::Pawn) = self.data.piece_from_square(from) {
                                add_pawn_block(v, from, dest, MoveType::DoublePush);
                            }
                        }
                    }
                    _ => {}
                }
            }
        };

        // Can we capture the attacker?
        for capturer in self.data.attacks_to(attacker_square, self.side) {
            let from = self.data.square_of_piece(capturer);
            v.push(Move::new(from, attacker_square, MoveType::Capture, None));
        }

        // Can we block the check?
        match attacker_piece {
            Piece::Bishop | Piece::Rook | Piece::Queen => {
                let direction = king_square.direction(attacker_square).unwrap();
                for dest in king_square.ray_attacks(direction) {
                    if dest == attacker_square {
                        break;
                    }

                    // Piece moves.
                    for attacker in self.data.attacks_to(dest, self.side)
                        & !self.data.pawns()
                        & !self.data.kings()
                    {
                        let from = self.data.square_of_piece(attacker);
                        v.push(Move::new(from, dest, MoveType::Normal, None));
                    }

                    // Pawn moves.
                    add_pawn_blocks(v, dest);
                }
            }
            _ => {}
        }
    }

    fn generate_double_check(&self, v: &mut ArrayVec<[Move; 256]>) {
        todo!("double check move generation");
    }

    /// Generate a vector of moves on the board.
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn generate(&self, v: &mut ArrayVec<[Move; 256]>) {
        // Unless something has gone very badly wrong we have to have a king.
        let king_index = (self.data.kings() & Bitlist::mask_from_colour(self.side))
            .peek()
            .expect("side to move has no king");
        let king_square = self.data.square_of_piece(king_index);
        let checks = self.data.attacks_to(king_square, !self.side);

        if checks.count_ones() == 1 {
            return self.generate_single_check(v);
        }

        if checks.count_ones() == 2 {
            return self.generate_double_check(v);
        }

        let pinned = self.generate_pinned_pieces(v);

        // General attack loop; pawns and kings handled separately.
        for dest in 0_u8..64 {
            // Squares will always be in range, so this will never panic.
            let dest = unsafe { Square::from_u8_unchecked(dest) };
            let mut kind = MoveType::Normal;

            // Is this a capture?
            if let Some(colour) = self.data.colour_from_square(dest) {
                if colour == self.side {
                    // Forbid own-colour captures.
                    continue;
                } else {
                    kind = MoveType::Capture;
                }
            }

            // For every piece that attacks this square, find its location and add it to the move list.
            for attacker in self.data.attacks_to(dest, self.side)
                & !self.data.pawns()
                & !self.data.kings()
                & !pinned
            {
                let from = self.data.square_of_piece(attacker);
                v.push(Move::new(from, dest, kind, None));
            }
        }

        // Pawns.
        self.generate_pawns(v, pinned);

        // King.
        self.generate_king(v);
    }
}

impl Drop for Board {
    fn drop(&mut self) {
        if ::std::thread::panicking() {
            println!("{}", self);
        }
    }
}