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
    square::{Direction, File, Rank, Square, Square16x8},
};
use std::{
    convert::{TryFrom, TryInto},
    ffi::CString,
    fmt::Display,
};

use tinyvec::ArrayVec;

mod bitlist;
mod data;
mod index;
mod piecelist;
mod piecemask;

use bitlist::Bitlist;
use data::BoardData;
pub use index::PieceIndex;

/// Pin information in a board.
pub struct PinInfo {
    pub pins: [Option<Direction>; 32],
    pub enpassant_pinned: Bitlist,
}

impl PinInfo {
    pub const fn new() -> Self {
        Self {
            pins: [None; 32],
            enpassant_pinned: Bitlist::new(),
        }
    }
}

impl Default for PinInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// A chess position.
#[derive(Clone)]
pub struct Board {
    /// The chess board representation.
    data: data::BoardData,
    /// The side to move.
    side: Colour,
    /// Castling rights, if any.
    castle: (bool, bool, bool, bool),
    /// En-passant square, if any.
    ep: Option<Square>,
}

impl Default for Board {
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

                write!(f, "{} ", c)?;
            } else {
                write!(f, ". ")?;
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
        if self.castle.0 {
            write!(f, "K")?;
        }
        if self.castle.1 {
            write!(f, "Q")?;
        }
        if self.castle.2 {
            write!(f, "k")?;
        }
        if self.castle.3 {
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
            castle: (false, false, false, false),
            ep: None,
            data: BoardData::new(),
        }
    }

    #[must_use]
    pub const fn side(&self) -> Colour {
        self.side
    }

    /// Check if this board is illegal by seeing if the enemy king is attacked by friendly pieces.
    /// If it is, it implies the move the enemy made left them in check, which is illegal.
    #[must_use]
    #[inline]
    pub fn illegal(&self) -> bool {
        #[allow(clippy::option_if_let_else)]
        if let Some(king_index) =
            (self.data.kings() & self.data.pieces_of_colour(!self.side)).peek()
        {
            let king_square = self.data.square_of_piece(king_index);
            return !self.data.attacks_to(king_square, self.side).empty();
        }
        // Not having a king is very definitely illegal.
        false
    }

    /// Parse a position in Forsyth-Edwards Notation into a board.
    #[must_use]
    pub fn from_fen(fen: &str) -> Option<Self> {
        let fen = CString::new(fen).expect("FEN is not ASCII");
        let fen = fen.as_bytes();
        Self::from_fen_bytes(fen)
    }

    /// Parse a position in Forsyth-Edwards Notation into a board.
    ///
    /// # Panics
    /// Panics when invalid FEN is input.
    #[must_use]
    pub fn from_fen_bytes(fen: &[u8]) -> Option<Self> {
        let mut b = Self::new();

        let mut idx = 0_usize;
        let mut c = fen[idx];

        for rank in (0..=7).rev() {
            let mut file = 0;
            while file <= 7 {
                if (b'1'..=b'8').contains(&c) {
                    let length = c - b'0';
                    let mut i = 0;
                    while i < length {
                        file += 1;
                        i += 1;
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

                    file += 1;
                }
                idx += 1;
                c = fen[idx];
            }
            if rank > 0 {
                idx += 1;
                c = fen[idx];
            }
        }
        idx += 1;
        c = fen[idx];
        b.side = match c {
            b'w' => Colour::White,
            b'b' => Colour::Black,
            _ => return None,
        };
        idx += 2;
        c = fen[idx];
        b.castle = (false, false, false, false);
        if c == b'-' {
            idx += 1;
        } else {
            if c == b'K' {
                b.castle.0 = true;
                idx += 1;
                c = fen[idx];
            }
            if c == b'Q' {
                b.castle.1 = true;
                idx += 1;
                c = fen[idx];
            }
            if c == b'k' {
                b.castle.2 = true;
                idx += 1;
                c = fen[idx];
            }
            if c == b'q' {
                b.castle.3 = true;
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
    ///
    /// # Panics
    /// Panics when Lofty hasn't implemented necessary code.
    #[inline]
    #[must_use]
    pub fn make(&self, m: Move) -> Self {
        let mut b = self.clone();
        match m.kind {
            MoveType::Normal => {
                b.data.move_piece(m.from, m.dest);
                b.ep = None;
            }
            MoveType::DoublePush => {
                b.data.move_piece(m.from, m.dest);
                b.ep = m.from.relative_north(b.side);
            }
            MoveType::Capture => {
                let piece_index = b
                    .data
                    .piece_index(m.dest)
                    .expect("attempted to capture an empty square");
                b.data.remove_piece(piece_index, true);
                b.data.move_piece(m.from, m.dest);
                b.ep = None;
            }
            MoveType::Castle => {
                if m.dest > m.from {
                    let rook_from = m.dest.east().unwrap();
                    let rook_to = m.dest.west().unwrap();
                    b.data.move_piece(rook_from, rook_to);
                } else {
                    let rook_from = m.dest.west().unwrap().west().unwrap();
                    let rook_to = m.dest.east().unwrap();
                    b.data.move_piece(rook_from, rook_to);
                }
                b.data.move_piece(m.from, m.dest);
                b.ep = None;
            }
            MoveType::EnPassant => {
                let target_square = b.ep.unwrap().relative_south(b.side).unwrap();
                let target_piece = b.data.piece_index(target_square).unwrap();
                b.data.remove_piece(target_piece, true);
                b.data.move_piece(m.from, m.dest);
                b.ep = None;
            }
            MoveType::Promotion => {
                let piece_index = b.data.piece_index(m.from).unwrap();
                b.data.remove_piece(piece_index, true);
                b.data.add_piece(m.prom.unwrap(), b.side, m.dest, true);
                b.ep = None;
            }
            MoveType::CapturePromotion => {
                let source_piece = b.data.piece_index(m.from).unwrap();
                let target_piece = b.data.piece_index(m.dest).unwrap();
                b.data.remove_piece(source_piece, true);
                b.data.remove_piece(target_piece, true);
                b.data.add_piece(m.prom.unwrap(), b.side, m.dest, true);
                b.ep = None;
            }
        }

        let a1 = Square::from_rank_file(Rank::One, File::A);
        let a8 = Square::from_rank_file(Rank::Eight, File::A);
        let e1 = Square::from_rank_file(Rank::One, File::E);
        let e8 = Square::from_rank_file(Rank::Eight, File::E);
        let h1 = Square::from_rank_file(Rank::One, File::H);
        let h8 = Square::from_rank_file(Rank::Eight, File::H);

        if m.from == e1 {
            b.castle.0 = false;
            b.castle.1 = false;
        }

        if m.from == e8 {
            b.castle.2 = false;
            b.castle.3 = false;
        }

        if m.from == h1 || m.dest == h1 {
            b.castle.0 = false;
        }

        if m.from == a1 || m.dest == a1 {
            b.castle.1 = false;
        }

        if m.from == h8 || m.dest == h8 {
            b.castle.2 = false;
        }

        if m.from == a8 || m.dest == a8 {
            b.castle.3 = false;
        }

        b.side = !b.side;
        b
    }

    fn try_push_move(
        &self,
        v: &mut ArrayVec<[Move; 256]>,
        from: Square,
        dest: Square,
        kind: MoveType,
        promotion_piece: Option<Piece>,
        pininfo: &PinInfo,
    ) {
        if let Some(dir) = pininfo.pins[self.data.piece_index(from).unwrap().into_inner() as usize]
        {
            if let Some(move_dir) = from.direction(dest) {
                // Pinned slider can only move along pin ray.
                if dir != move_dir && dir != move_dir.opposite() {
                    return;
                }
            } else {
                // Pinned knight can't move.
                return;
            }
        }
        v.push(Move::new(from, dest, kind, promotion_piece));
    }

    /// Find pinned pieces and handle them specially.
    ///
    /// # Panics
    /// Panics when Lofty has written shitty code.
    #[must_use]
    pub fn discover_pinned_pieces(&self) -> PinInfo {
        let mut info = PinInfo::new();

        let sliders = self.data.bishops() | self.data.rooks() | self.data.queens();
        let king_index = (self.data.kings() & Bitlist::mask_from_colour(self.side))
            .peek()
            .unwrap();
        let king_square = self.data.square_of_piece(king_index);
        let king_square_16x8 = Square16x8::from_square(king_square);

        for possible_pinner in self.data.pieces_of_colour(!self.side).and(sliders) {
            let pinner_square = self.data.square_of_piece(possible_pinner);
            let pinner_square_16x8 = Square16x8::from_square(pinner_square);
            let pinner_type = self.data.piece_from_bit(possible_pinner);
            let pinner_king_dir = match pinner_square_16x8.direction(king_square_16x8) {
                Some(dir) => dir,
                None => continue,
            };

            if !pinner_king_dir.valid_for_slider(pinner_type) {
                continue;
            }

            let mut friendly_blocker = None;
            let mut enemy_blocker = None;
            for square in pinner_square_16x8.ray_attacks(pinner_king_dir) {
                if square == king_square {
                    break;
                }

                if let Some(piece_index) = self.data.piece_index(square) {
                    if self.data.colour_from_square(square) == Some(!self.side) {
                        match enemy_blocker {
                            Some(_) => {
                                friendly_blocker = None;
                                enemy_blocker = None;
                                break;
                            }
                            None => {
                                enemy_blocker = Some(piece_index);
                            }
                        }
                    } else {
                        match friendly_blocker {
                            Some(_) => {
                                friendly_blocker = None;
                                enemy_blocker = None;
                                break;
                            }
                            None => {
                                friendly_blocker = Some(piece_index);
                            }
                        }
                    }
                }
            }

            match (friendly_blocker, enemy_blocker) {
                // There are no friendly blockers: skip.
                (None, _) => continue,
                // There is one friendly blocker: it is pinned.
                (Some(blocker), None) => {
                    info.pins[blocker.into_inner() as usize] = Some(pinner_king_dir);
                }
                // There is one friendly blocker and one enemy blocker: it *may* be pinned for en-passant purposes
                (Some(friendly_blocker), Some(enemy_blocker)) => {
                    // If at least one of the blockers is a piece, we don't need to worry about en-passant.
                    if self.data.piece_from_bit(friendly_blocker) != Piece::Pawn
                        || self.data.piece_from_bit(enemy_blocker) != Piece::Pawn
                        || (pinner_king_dir != Direction::East
                            && pinner_king_dir != Direction::West)
                    {
                        continue;
                    }

                    // Alas, we do have to care.
                    info.enpassant_pinned |= Bitlist::from(friendly_blocker);
                }
            }
        }

        info
    }

    /// Generate en-passant pawn moves.
    fn generate_pawn_enpassant(&self, v: &mut ArrayVec<[Move; 256]>, pininfo: &PinInfo) {
        if let Some(ep) = self.ep {
            for capturer in self
                .data
                .attacks_to(ep, self.side)
                .and(self.data.pawns())
                .and(!pininfo.enpassant_pinned)
            {
                let from = self.data.square_of_piece(capturer);
                self.try_push_move(v, from, ep, MoveType::EnPassant, None, pininfo);
            }
        }
    }

    /// Generate pawn-specific quiet moves.
    fn generate_pawn_quiet(&self, v: &mut ArrayVec<[Move; 256]>, from: Square, pininfo: &PinInfo) {
        let north = from.relative_north(self.side);
        if let Some(dest) = north {
            // Pawn single pushes.
            if !self.data.has_piece(dest) {
                if Rank::from(dest).is_relative_eighth(self.side) {
                    self.try_push_move(
                        v,
                        from,
                        dest,
                        MoveType::Promotion,
                        Some(Piece::Queen),
                        pininfo,
                    );
                    self.try_push_move(
                        v,
                        from,
                        dest,
                        MoveType::Promotion,
                        Some(Piece::Knight),
                        pininfo,
                    );
                    self.try_push_move(
                        v,
                        from,
                        dest,
                        MoveType::Promotion,
                        Some(Piece::Rook),
                        pininfo,
                    );
                    self.try_push_move(
                        v,
                        from,
                        dest,
                        MoveType::Promotion,
                        Some(Piece::Bishop),
                        pininfo,
                    );
                } else {
                    self.try_push_move(v, from, dest, MoveType::Normal, None, pininfo);
                }

                // Pawn double pushes.
                let north2 = dest.relative_north(self.side);
                if let Some(dest) = north2 {
                    if Rank::from(dest).is_relative_fourth(self.side) && !self.data.has_piece(dest)
                    {
                        self.try_push_move(v, from, dest, MoveType::DoublePush, None, pininfo);
                    }
                }
            }
        }
    }

    /// Generate moves when in check by a single piece.
    #[allow(clippy::too_many_lines)]
    fn generate_single_check(&self, v: &mut ArrayVec<[Move; 256]>) {
        #[allow(clippy::unwrap_used)]
        let king_index = (self.data.kings() & Bitlist::mask_from_colour(self.side))
            .peek()
            .unwrap();
        let king_square = self.data.square_of_piece(king_index);
        let king_square_16x8 = Square16x8::from_square(king_square);
        let attacker_bit = self.data.attacks_to(king_square, !self.side);
        let attacker_index = attacker_bit.peek().unwrap();
        let attacker_piece = self.data.piece_from_bit(attacker_index);
        let attacker_square = self.data.square_of_piece(attacker_index);
        let attacker_direction = attacker_square.direction(king_square);

        let pininfo = self.discover_pinned_pieces();

        let add_pawn_block = |v: &mut ArrayVec<[Move; 256]>, from, dest, kind| {
            if let Some(colour) = self.data.colour_from_square(from) {
                if colour == self.side {
                    self.try_push_move(v, from, dest, kind, None, &pininfo);
                }
            }
        };

        let add_pawn_blocks = |v: &mut ArrayVec<[Move; 256]>, dest: Square| {
            if let Some(from) = dest.relative_south(self.side) {
                match self.data.piece_from_square(from) {
                    Some(Piece::Pawn) => add_pawn_block(v, from, dest, MoveType::Normal),
                    Some(_) => {}
                    None => {
                        if Rank::from(dest).is_relative_fourth(self.side) {
                            if let Some(from) = from.relative_south(self.side) {
                                if let Some(Piece::Pawn) = self.data.piece_from_square(from) {
                                    add_pawn_block(v, from, dest, MoveType::DoublePush);
                                }
                            }
                        }
                    }
                }
            }
        };

        // Can we capture the attacker?
        for capturer in self.data.attacks_to(attacker_square, self.side) {
            let from = self.data.square_of_piece(capturer);
            if self.data.piece_from_bit(capturer) == Piece::King
                && !self.data.attacks_to(attacker_square, !self.side).empty()
            {
                continue;
            }
            if self.data.piece_from_bit(capturer) == Piece::Pawn
                && Rank::from(attacker_square).is_relative_eighth(self.side)
            {
                self.try_push_move(
                    v,
                    from,
                    attacker_square,
                    MoveType::CapturePromotion,
                    Some(Piece::Queen),
                    &pininfo,
                );
                self.try_push_move(
                    v,
                    from,
                    attacker_square,
                    MoveType::CapturePromotion,
                    Some(Piece::Knight),
                    &pininfo,
                );
                self.try_push_move(
                    v,
                    from,
                    attacker_square,
                    MoveType::CapturePromotion,
                    Some(Piece::Rook),
                    &pininfo,
                );
                self.try_push_move(
                    v,
                    from,
                    attacker_square,
                    MoveType::CapturePromotion,
                    Some(Piece::Bishop),
                    &pininfo,
                );
            } else {
                self.try_push_move(v, from, attacker_square, MoveType::Capture, None, &pininfo);
            }
        }

        if let Some(ep) = self.ep {
            if let Some(ep_south) = ep.relative_south(self.side) {
                if ep_south == attacker_square && attacker_piece == Piece::Pawn {
                    for capturer in self.data.attacks_to(ep, self.side)
                        & self.data.pawns()
                        & !pininfo.enpassant_pinned
                    {
                        self.try_push_move(
                            v,
                            self.data.square_of_piece(capturer),
                            ep,
                            MoveType::EnPassant,
                            None,
                            &pininfo,
                        );
                    }
                }
            }
        }

        // Can we block the check?
        if let Piece::Bishop | Piece::Rook | Piece::Queen = attacker_piece {
            let direction = king_square.direction(attacker_square).unwrap();
            for dest in king_square_16x8.ray_attacks(direction) {
                if dest == attacker_square {
                    break;
                }

                // Piece moves.
                for attacker in self
                    .data
                    .attacks_to(dest, self.side)
                    .and(!self.data.pawns())
                    .and(!self.data.kings())
                {
                    self.try_push_move(
                        v,
                        self.data.square_of_piece(attacker),
                        dest,
                        MoveType::Normal,
                        None,
                        &pininfo,
                    );
                }

                // Pawn moves.
                add_pawn_blocks(v, dest);
            }
        }

        // Can we move the king?
        for square in king_square.king_attacks() {
            let kind = if self.data.has_piece(square) {
                if square == attacker_square
                    || self.data.colour_from_square(square) == Some(self.side)
                {
                    // Own-piece captures are illegal, captures of the attacker are handled elsewhere.
                    continue;
                }
                MoveType::Capture
            } else {
                MoveType::Normal
            };

            if !self.data.attacks_to(square, !self.side).empty() {
                // Moving into check is illegal.
                continue;
            }
            if let Some(attacker_direction) = attacker_direction {
                // Slider attacks x-ray through the king to attack that square.
                if let Some(xray_square) = king_square.travel(attacker_direction) {
                    if matches!(attacker_piece, Piece::Bishop | Piece::Rook | Piece::Queen)
                        && xray_square == square
                    {
                        continue;
                    }
                }
            }

            v.push(Move::new(king_square, square, kind, None));
        }
    }

    fn generate_double_check(&self, v: &mut ArrayVec<[Move; 256]>) {
        #[allow(clippy::unwrap_used)]
        let king_index = (self.data.kings() & Bitlist::mask_from_colour(self.side))
            .peek()
            .unwrap();
        let king_square = self.data.square_of_piece(king_index);
        let mut attacker_bits = self.data.attacks_to(king_square, !self.side);
        let attacker1_index = attacker_bits.pop().unwrap();
        let attacker1_piece = self.data.piece_from_bit(attacker1_index);
        let attacker1_square = self.data.square_of_piece(attacker1_index);
        let attacker1_direction = attacker1_square.direction(king_square);
        let attacker2_index = attacker_bits.pop().unwrap();
        let attacker2_piece = self.data.piece_from_bit(attacker2_index);
        let attacker2_square = self.data.square_of_piece(attacker2_index);
        let attacker2_direction = attacker2_square.direction(king_square);

        // Can we move the king?
        for square in king_square.king_attacks() {
            let kind = if self.data.has_piece(square) {
                if self.data.colour_from_square(square) == Some(self.side) {
                    // Own-piece captures are illegal.
                    continue;
                }
                MoveType::Capture
            } else {
                MoveType::Normal
            };

            if !self.data.attacks_to(square, !self.side).empty() {
                // Moving into check is illegal.
                continue;
            }

            // Slider attacks x-ray through the king to attack that square.
            if let Some(attacker1_direction) = attacker1_direction {
                if let Some(xray_square) = king_square.travel(attacker1_direction) {
                    if matches!(attacker1_piece, Piece::Bishop | Piece::Rook | Piece::Queen)
                        && xray_square == square
                    {
                        continue;
                    }
                }
            }

            if let Some(attacker2_direction) = attacker2_direction {
                if let Some(xray_square) = king_square.travel(attacker2_direction) {
                    if matches!(attacker2_piece, Piece::Bishop | Piece::Rook | Piece::Queen)
                        && xray_square == square
                    {
                        continue;
                    }
                }
            }

            v.push(Move::new(king_square, square, kind, None));
        }
    }

    pub fn generate_captures(&self, v: &mut ArrayVec<[Move; 256]>) {
        let pininfo = self.discover_pinned_pieces();

        let mut find_attackers = |dest: Square| {
            let attacks = self.data.attacks_to(dest, self.side);
            for capturer in attacks & self.data.pawns() {
                let from = self.data.square_of_piece(capturer);
                if Rank::from(dest).is_relative_eighth(self.side) {
                    self.try_push_move(
                        v,
                        from,
                        dest,
                        MoveType::CapturePromotion,
                        Some(Piece::Queen),
                        &pininfo,
                    );
                    self.try_push_move(
                        v,
                        from,
                        dest,
                        MoveType::CapturePromotion,
                        Some(Piece::Knight),
                        &pininfo,
                    );
                    self.try_push_move(
                        v,
                        from,
                        dest,
                        MoveType::CapturePromotion,
                        Some(Piece::Rook),
                        &pininfo,
                    );
                    self.try_push_move(
                        v,
                        from,
                        dest,
                        MoveType::CapturePromotion,
                        Some(Piece::Bishop),
                        &pininfo,
                    );
                } else {
                    self.try_push_move(v, from, dest, MoveType::Capture, None, &pininfo);
                }
            }
            for capturer in attacks & self.data.knights() {
                let from = self.data.square_of_piece(capturer);
                self.try_push_move(v, from, dest, MoveType::Capture, None, &pininfo);
            }
            for capturer in attacks & self.data.bishops() {
                let from = self.data.square_of_piece(capturer);
                self.try_push_move(v, from, dest, MoveType::Capture, None, &pininfo);
            }
            for capturer in attacks & self.data.rooks() {
                let from = self.data.square_of_piece(capturer);
                self.try_push_move(v, from, dest, MoveType::Capture, None, &pininfo);
            }
            for capturer in attacks & self.data.queens() {
                let from = self.data.square_of_piece(capturer);
                self.try_push_move(v, from, dest, MoveType::Capture, None, &pininfo);
            }
            for capturer in attacks & self.data.kings() {
                let from = self.data.square_of_piece(capturer);
                if !self.data.attacks_to(dest, !self.side).empty() {
                    // Moving into check is illegal.
                    continue;
                }
                self.try_push_move(v, from, dest, MoveType::Capture, None, &pininfo);
            }
        };

        for victim in self.data.pieces_of_colour(!self.side) & self.data.queens() {
            find_attackers(self.square_of_piece(victim));
        }
        for victim in self.data.pieces_of_colour(!self.side) & self.data.rooks() {
            find_attackers(self.square_of_piece(victim));
        }
        for victim in self.data.pieces_of_colour(!self.side) & self.data.bishops() {
            find_attackers(self.square_of_piece(victim));
        }
        for victim in self.data.pieces_of_colour(!self.side) & self.data.knights() {
            find_attackers(self.square_of_piece(victim));
        }
        for victim in self.data.pieces_of_colour(!self.side) & self.data.pawns() {
            find_attackers(self.square_of_piece(victim));
        }

        self.generate_pawn_enpassant(v, &pininfo);
    }

    /// Generate a vector of moves on the board.
    ///
    /// # Panics
    /// Panics when Lofty writes shitty code.
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

        let pininfo = self.discover_pinned_pieces();
        self.generate_captures(v);

        // Pawns.
        for pawn in self.data.pawns().and(Bitlist::mask_from_colour(self.side)) {
            let from = self.data.square_of_piece(pawn);
            self.generate_pawn_quiet(v, from, &pininfo);
        }

        // General quiet move loop; pawns and kings handled separately.
        for dest in 0_u8..64 {
            // Squares will always be in range, so this will never panic.
            let dest = unsafe { Square::from_u8_unchecked(dest) };

            // Ignore captures.
            if self.data.has_piece(dest) {
                continue;
            }

            // For every piece that attacks this square, find its location and add it to the move list.
            for attacker in self
                .data
                .attacks_to(dest, self.side)
                .and(!self.data.pawns())
            //.and(!self.data.kings())
            {
                // It's illegal for kings to move to attacked squares; prune those out.
                if self.data.piece_from_bit(attacker) == Piece::King
                    && !self.data.attacks_to(dest, !self.side).empty()
                {
                    continue;
                }

                let from = self.data.square_of_piece(attacker);
                self.try_push_move(v, from, dest, MoveType::Normal, None, &pininfo);
            }
        }

        // Kingside castling.
        if (self.side == Colour::White && self.castle.0)
            || (self.side == Colour::Black && self.castle.2)
        {
            let east1 = king_square.east().unwrap();
            let east2 = east1.east().unwrap();
            if self.data.attacks_to(king_square, !self.side).empty()
                && !self.data.has_piece(east1)
                && self.data.attacks_to(east1, !self.side).empty()
                && !self.data.has_piece(east2)
                && self.data.attacks_to(east2, !self.side).empty()
            {
                self.try_push_move(v, king_square, east2, MoveType::Castle, None, &pininfo);
            }
        }

        // Queenside castling.
        if (self.side == Colour::White && self.castle.1)
            || (self.side == Colour::Black && self.castle.3)
        {
            let west1 = king_square.west().unwrap();
            let west2 = west1.west().unwrap();
            let west3 = west2.west().unwrap();
            if self.data.attacks_to(king_square, !self.side).empty()
                && !self.data.has_piece(west1)
                && self.data.attacks_to(west1, !self.side).empty()
                && !self.data.has_piece(west2)
                && self.data.attacks_to(west2, !self.side).empty()
                && !self.data.has_piece(west3)
            {
                self.try_push_move(v, king_square, west2, MoveType::Castle, None, &pininfo);
            }
        }
    }

    #[must_use]
    pub const fn kings(&self) -> Bitlist {
        self.data.kings()
    }

    /// Return a bitlist of all pieces.
    #[must_use]
    pub const fn pieces(&self) -> Bitlist {
        self.data.pieces()
    }

    /// Given a piece index, return its piece type.
    #[must_use]
    pub fn piece_from_bit(&self, bit: PieceIndex) -> Piece {
        self.data.piece_from_bit(bit)
    }

    #[must_use]
    pub fn square_of_piece(&self, bit: PieceIndex) -> Square {
        self.data.square_of_piece(bit)
    }
}

/* impl Drop for Board {
    fn drop(&mut self) {
        if ::std::thread::panicking() {
            println!("{}", self);
        }
    }
} */
