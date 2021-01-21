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

use std::{convert::TryFrom, fmt::Display, num::NonZeroU8};
use crate::colour::Colour;

/// A chessboard rank.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
        }
    }
}

impl From<Rank> for u8 {
    #[inline]
    fn from(rank: Rank) -> Self {
        match rank {
            Rank::One => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::One),
            1 => Ok(Self::Two),
            2 => Ok(Self::Three),
            3 => Ok(Self::Four),
            4 => Ok(Self::Five),
            5 => Ok(Self::Six),
            6 => Ok(Self::Seven),
            7 => Ok(Self::Eight),
            _ => Err(()),
        }
    }
}

impl Rank {
    pub const fn north(self) -> Option<Self> {
        match self {
            Self::One => Some(Self::Two),
            Self::Two => Some(Self::Three),
            Self::Three => Some(Self::Four),
            Self::Four => Some(Self::Five),
            Self::Five => Some(Self::Six),
            Self::Six => Some(Self::Seven),
            Self::Seven => Some(Self::Eight),
            Self::Eight => None,
        }
    }

    pub const fn south(self) -> Option<Self> {
        match self {
            Self::One => None,
            Self::Two => Some(Self::One),
            Self::Three => Some(Self::Two),
            Self::Four => Some(Self::Three),
            Self::Five => Some(Self::Four),
            Self::Six => Some(Self::Five),
            Self::Seven => Some(Self::Six),
            Self::Eight => Some(Self::Seven),
        }
    }

    pub fn is_relative_fourth(self, colour: Colour) -> bool {
        match colour {
            Colour::White => self == Self::Four,
            Colour::Black => self == Self::Five,
        }
    }

    pub fn is_relative_eighth(self, colour: Colour) -> bool {
        match colour {
            Colour::White => self == Self::Eight,
            Colour::Black => self == Self::One,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "a"),
            Self::B => write!(f, "b"),
            Self::C => write!(f, "c"),
            Self::D => write!(f, "d"),
            Self::E => write!(f, "e"),
            Self::F => write!(f, "f"),
            Self::G => write!(f, "g"),
            Self::H => write!(f, "h"),
        }
    }
}

impl From<File> for u8 {
    #[inline]
    fn from(file: File) -> Self {
        match file {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
}

impl TryFrom<u8> for File {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::D),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            6 => Ok(Self::G),
            7 => Ok(Self::H),
            _ => Err(()),
        }
    }
}

impl File {
    pub const fn east(self) -> Option<Self> {
        match self {
            Self::A => Some(Self::B),
            Self::B => Some(Self::C),
            Self::C => Some(Self::D),
            Self::D => Some(Self::E),
            Self::E => Some(Self::F),
            Self::F => Some(Self::G),
            Self::G => Some(Self::H),
            Self::H => None,
        }
    }

    pub const fn west(self) -> Option<Self> {
        match self {
            Self::A => None,
            Self::B => Some(Self::A),
            Self::C => Some(Self::B),
            Self::D => Some(Self::C),
            Self::E => Some(Self::D),
            Self::F => Some(Self::E),
            Self::G => Some(Self::F),
            Self::H => Some(Self::G),
        }
    }
}

/// A square on a chessboard.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Square(NonZeroU8);

impl Default for Square {
    fn default() -> Self {
        // SAFETY: One is not zero.
        Self(unsafe { NonZeroU8::new_unchecked(1) })
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", File::from(*self), Rank::from(*self))
    }
}

impl From<Square> for Rank {
    fn from(square: Square) -> Self {
        // This is an exhaustive match, so the unreachable! really is unreachable.
        #[allow(clippy::unreachable)]
        match square.into_inner() / 8 {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Three,
            3 => Self::Four,
            4 => Self::Five,
            5 => Self::Six,
            6 => Self::Seven,
            7 => Self::Eight,
            _ => unreachable!(),
        }
    }
}

impl From<Square> for File {
    fn from(square: Square) -> Self {
        // This is an exhaustive match, so the unreachable! really is unreachable.
        #[allow(clippy::unreachable)]
        match square.into_inner() % 8 {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            5 => Self::F,
            6 => Self::G,
            7 => Self::H,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for Square {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self::from_rank_file(
            Rank::try_from(value / 8)?,
            File::try_from(value % 8)?,
        ))
    }
}

impl Square {
    /// Construct a `Square` from a `Rank` and `File`.
    #[must_use]
    pub fn from_rank_file(rank: Rank, file: File) -> Self {
        let rank = u8::from(rank);
        let file = u8::from(file);
        // SAFETY: the "plus one" ensures this will never be zero.
        let square = unsafe { NonZeroU8::new_unchecked((8 * rank + file) + 1) };
        Self(square)
    }

    /// Construct a `Square` directly from a `u8`.
    ///
    /// # Safety
    ///
    /// `sq` must be in the range 0-63.
    #[must_use]
    pub const unsafe fn from_u8_unchecked(sq: u8) -> Self {
        Self(NonZeroU8::new_unchecked(sq + 1))
    }

    /// Return the internal `u8`.
    #[must_use]
    pub const fn into_inner(self) -> u8 {
        // The "& 63" is to hint to the compiler that this will never be greater than it.
        (self.0.get() - 1) & 63
    }

    /// Return the `Direction` between two squares, if any exists.
    #[must_use]
    pub fn direction(self, dest: Self) -> Option<Direction> {
        const DIRECTIONS: [Option<Direction>; 240] = [
            Some(Direction::SouthWest), None, None, None, None, None, None, Some(Direction::South), None, None, None, None, None, None, Some(Direction::SouthEast), None, None, Some(Direction::SouthWest), None, None, None, None, None, Some(Direction::South), None, None, None, None, None, Some(Direction::SouthEast), None, None, None, None, Some(Direction::SouthWest), None, None, None, None, Some(Direction::South), None, None, None, None, Some(Direction::SouthEast), None, None, None, None, None, None, Some(Direction::SouthWest), None, None, None, Some(Direction::South), None, None, None, Some(Direction::SouthEast), None, None, None, None, None, None, None, None, Some(Direction::SouthWest), None, None, Some(Direction::South), None, None, Some(Direction::SouthEast), None, None, None, None, None, None, None, None, None, None, Some(Direction::SouthWest), None, Some(Direction::South), None, Some(Direction::SouthEast), None, None, None, None, None, None, None, None, None, None, None, None, Some(Direction::SouthWest), Some(Direction::South), Some(Direction::SouthEast), None, None, None, None, None, None, None, Some(Direction::West), Some(Direction::West), Some(Direction::West), Some(Direction::West), Some(Direction::West), Some(Direction::West), Some(Direction::West), None, Some(Direction::East), Some(Direction::East), Some(Direction::East), Some(Direction::East), Some(Direction::East), Some(Direction::East), Some(Direction::East), None, None, None, None, None, None, None, Some(Direction::NorthWest), Some(Direction::North), Some(Direction::NorthEast), None, None, None, None, None, None, None, None, None, None, None, None, Some(Direction::NorthWest), None, Some(Direction::North), None, Some(Direction::NorthEast), None, None, None, None, None, None, None, None, None, None, Some(Direction::NorthWest), None, None, Some(Direction::North), None, None, Some(Direction::NorthEast), None, None, None, None, None, None, None, None, Some(Direction::NorthWest), None, None, None, Some(Direction::North), None, None, None, Some(Direction::NorthEast), None, None, None, None, None, None, Some(Direction::NorthWest), None, None, None, None, Some(Direction::North), None, None, None, None, Some(Direction::NorthEast), None, None, None, None, Some(Direction::NorthWest), None, None, None, None, None, Some(Direction::North), None, None, None, None, None, Some(Direction::NorthEast), None, None, Some(Direction::NorthWest), None, None, None, None, None, None, Some(Direction::North), None, None, None, None, None, None, Some(Direction::NorthEast), None,
        ];

        let to_16x12 = |sq: Self| 16 * (sq.into_inner() / 8) + (sq.into_inner() % 8) + 36;

        let dest = to_16x12(dest);
        let from = to_16x12(self);

        unsafe {
            *DIRECTIONS.get_unchecked(usize::from(
                dest
                    .wrapping_sub(from)
                    .wrapping_add(119),
            ))
        }
    }

    /// Return the `Square` in a given `Direction`, if one exists.
    #[must_use]
    pub const fn travel(self, direction: Direction) -> Option<Self> {
        const fn to_16x8(square: Square) -> i16 {
            let square = square.into_inner();
            (square + (square & !7)) as i16
        }

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
        let square_8x8 = (self.into_inner() as i8 + direction.to_8x8()) as u8;
        let square_16x8 = to_16x8(self).wrapping_add(direction.to_16x8());

        if (square_16x8 & 0x88) == 0 {
            unsafe {
                return Some(Self::from_u8_unchecked(square_8x8));
            }
        }
        None
    }

    #[must_use]
    pub const fn north(self) -> Option<Self> {
        self.travel(Direction::North)
    }

    #[must_use]
    pub const fn north_east(self) -> Option<Self> {
        self.travel(Direction::NorthEast)
    }

    #[must_use]
    pub const fn east(self) -> Option<Self> {
        self.travel(Direction::East)
    }

    #[must_use]
    pub const fn south_east(self) -> Option<Self> {
        self.travel(Direction::SouthEast)
    }

    #[must_use]
    pub const fn south(self) -> Option<Self> {
        self.travel(Direction::South)
    }

    #[must_use]
    pub const fn south_west(self) -> Option<Self> {
        self.travel(Direction::SouthWest)
    }

    #[must_use]
    pub const fn west(self) -> Option<Self> {
        self.travel(Direction::West)
    }

    #[must_use]
    pub const fn north_west(self) -> Option<Self> {
        self.travel(Direction::NorthWest)
    }

    /// The colour-dependent north of a square.
    #[must_use]
    pub const fn relative_north(self, colour: Colour) -> Option<Self> {
        match colour {
            Colour::White => self.north(),
            Colour::Black => self.south(),
        }
    }

    /// The colour-dependent south of a square.
    #[must_use]
    pub const fn relative_south(self, colour: Colour) -> Option<Self> {
        match colour {
            Colour::White => self.south(),
            Colour::Black => self.north(),
        }
    }

    /// An iterator over the squares a pawn attacks.
    #[must_use]
    pub const fn pawn_attacks(self, colour: Colour) -> PawnIter {
        let relative_north = match colour {
            Colour::White => self.north(),
            Colour::Black => self.south(),
        };

        PawnIter(relative_north, 0)
    }

    /// An iterator over the squares a knight attacks.
    #[must_use]
    pub const fn knight_attacks(self) -> KnightIter {
        KnightIter(self, 0)
    }

    /// An iterator over the squares a king attacks.
    #[must_use]
    pub const fn king_attacks(self) -> KingIter {
        KingIter(self, 0)
    }

    /// An iterator over the squares in a `Direction`.
    #[must_use]
    pub const fn ray_attacks(self, dir: Direction) -> RayIter {
        RayIter(self, dir)
    }
}

/// A chess direction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    /// North.
    North,
    /// North-northeast.
    NorthNorthEast,
    /// Northeast.
    NorthEast,
    /// East-northeast.
    EastNorthEast,
    /// East.
    East,
    /// East-southeast.
    EastSouthEast,
    /// Southeast.
    SouthEast,
    /// South-southeast.
    SouthSouthEast,
    /// South.
    South,
    /// South-southwest.
    SouthSouthWest,
    /// Southwest.
    SouthWest,
    /// West-southwest.
    WestSouthWest,
    /// West.
    West,
    /// West-northwest.
    WestNorthWest,
    /// Northwest.
    NorthWest,
    /// North-northwest.
    NorthNorthWest,
}

impl Direction {
    /// The `Direction` 180 degrees of the given `Direction`.
    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::East => Self::West,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::SouthEast,
            Self::NorthNorthEast => Self::SouthSouthWest,
            Self::EastNorthEast => Self::WestSouthWest,
            Self::EastSouthEast => Self::WestNorthWest,
            Self::SouthSouthEast => Self::NorthNorthWest,
            Self::SouthSouthWest => Self::NorthNorthEast,
            Self::WestSouthWest => Self::EastNorthEast,
            Self::WestNorthWest => Self::EastSouthEast,
            Self::NorthNorthWest => Self::SouthSouthEast,
        }
    }

    /// Returns true if the direction is diagonal.
    pub const fn diagonal(self) -> bool {
        matches!(self, Self::NorthEast | Self::SouthEast | Self::SouthWest | Self::NorthWest)
    }

    /// Return true if the direction is orthogonal.
    pub const fn orthogonal(self) -> bool {
        matches!(self, Self::North | Self::East | Self::West | Self::South)
    }

    /// Returns the 16x8 square difference of this Direction.
    pub const fn to_16x8(self) -> i16 {
        match self {
            Self::North => 16,
            Self::NorthEast => 17,
            Self::East => 1,
            Self::SouthEast => -15,
            Self::South => -16,
            Self::SouthWest => -17,
            Self::West => -1,
            Self::NorthWest => 15,
            Self::NorthNorthEast => 33,
            Self::EastNorthEast => 18,
            Self::EastSouthEast => -14,
            Self::SouthSouthEast => -31,
            Self::SouthSouthWest => -33,
            Self::WestSouthWest => -18,
            Self::WestNorthWest => 14,
            Self::NorthNorthWest => 31,
        }
    }

    pub const fn to_8x8(self) -> i8 {
        match self {
            Self::North => 8,
            Self::NorthEast => 9,
            Self::East => 1,
            Self::SouthEast => -7,
            Self::South => -8,
            Self::SouthWest => -9,
            Self::West => -1,
            Self::NorthWest => 7,
            Self::NorthNorthEast => 17,
            Self::EastNorthEast => 10,
            Self::EastSouthEast => -6,
            Self::SouthSouthEast => -15,
            Self::SouthSouthWest => -17,
            Self::WestSouthWest => -10,
            Self::WestNorthWest => 6,
            Self::NorthNorthWest => 15,
        }
    }
}

pub struct PawnIter(Option<Square>, u8);

impl Iterator for PawnIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match self.1 {
                0 => self.0.and_then(Square::east),
                1 => self.0.and_then(Square::west),
                _ => return None,
            };

            self.1 += 1;

            if next.is_some() {
                return next;
            }
        }
    }
}

/// An iterator over the knight attacks of a `Square`.
pub struct KnightIter(Square, u8);

impl Iterator for KnightIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match self.1 {
                0 => self.0.travel(Direction::NorthNorthEast),
                1 => self.0.travel(Direction::EastNorthEast),
                2 => self.0.travel(Direction::EastSouthEast),
                3 => self.0.travel(Direction::SouthSouthEast),
                4 => self.0.travel(Direction::SouthSouthWest),
                5 => self.0.travel(Direction::WestSouthWest),
                6 => self.0.travel(Direction::WestNorthWest),
                7 => self.0.travel(Direction::NorthNorthWest),
                _ => return None,
            };

            self.1 += 1;

            if next.is_some() {
                return next;
            }
        }
    }
}

/// An iterator over the `Square`s in a `Direction`.
pub struct RayIter(Square, Direction);

impl Iterator for RayIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.travel(self.1)?;
        self.0 = next;
        Some(next)
    }
}

/// An iterator over the king attacks of a `Square`.
pub struct KingIter(Square, u8);

impl Iterator for KingIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match self.1 {
                0 => self.0.north(),
                1 => self.0.north_east(),
                2 => self.0.east(),
                3 => self.0.south_east(),
                4 => self.0.south(),
                5 => self.0.south_west(),
                6 => self.0.west(),
                7 => self.0.north_west(),
                _ => return None,
            };

            self.1 += 1;

            if next.is_some() {
                return next;
            }
        }
    }
}
