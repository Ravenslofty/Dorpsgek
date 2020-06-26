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
    pub fn north(self) -> Option<Self> {
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

    pub fn south(self) -> Option<Self> {
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
    pub fn east(self) -> Option<Self> {
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

    pub fn west(self) -> Option<Self> {
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
    pub const unsafe fn from_u8_unchecked(sq: u8) -> Self {
        Self(NonZeroU8::new_unchecked(sq + 1))
    }

    /// Return the internal `u8`.
    pub const fn into_inner(self) -> u8 {
        // The "& 63" is to hint to the compiler that this will never be greater than it.
        (self.0.get() - 1) & 63
    }

    /// Return the `Direction` between two squares, if any exists.
    pub fn direction(self, dest: Self) -> Option<Direction> {
        /// Whether DIRECTIONS has been initialised.
        static mut INIT: bool = false;
        /// Lazily-initialised direction table using 16x12 coordinates.
        static mut DIRECTIONS: [Option<Direction>; 240] = [None; 240];

        let to_16x12 = |sq: Self| ((16 * u8::from(Rank::from(sq))) + u8::from(File::from(sq)) + 36);

        unsafe {
            if !INIT {
                let a1 = Self::from_rank_file(Rank::One, File::A);
                let h1 = Self::from_rank_file(Rank::One, File::H);
                let a8 = Self::from_rank_file(Rank::Eight, File::A);
                let h8 = Self::from_rank_file(Rank::Eight, File::H);

                let travel = |src, dir| {
                    let src_16x12 = to_16x12(src);
                    for dest in src.ray_attacks(dir) {
                        let dest_16x12 = to_16x12(dest);
                        let entry = DIRECTIONS.get_mut(usize::from(
                            dest_16x12.wrapping_sub(src_16x12).wrapping_add(119),
                        ));

                        if let Some(entry) = entry {
                            *entry = Some(dir);
                        }
                    }
                };

                travel(a1, Direction::North);
                travel(a1, Direction::NorthEast);
                travel(a1, Direction::East);
                travel(a8, Direction::SouthEast);
                travel(h8, Direction::South);
                travel(h8, Direction::SouthWest);
                travel(h8, Direction::West);
                travel(h1, Direction::NorthWest);

                INIT = true;
            }

            *DIRECTIONS.get(usize::from(
                to_16x12(dest)
                    .wrapping_sub(to_16x12(self))
                    .wrapping_add(119),
            ))?
        }
    }

    /// Return the `Square` in a given `Direction`, if one exists.
    pub fn travel(self, direction: Direction) -> Option<Self> {
        /// 16x12 to 8x8 conversion table.
        static FROM_16X12: [Option<u8>; 192] = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, None, None, None,
            None, None, None, None, Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, None, None, None,
            None, None, None, None, Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), None, None, None, None,
            None, None, None, None, Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), None, None, None, None,
            None, None, None, None, Some(32), Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), None, None, None, None,
            None, None, None, None, Some(40), Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), Some(47), None, None, None, None,
            None, None, None, None, Some(48), Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), Some(55), None, None, None, None,
            None, None, None, None, Some(56), Some(57), Some(58), Some(59), Some(60), Some(61), Some(62), Some(63), None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,   
        ];

        let to_16x12 = |sq: Self| ((16 * u8::from(Rank::from(sq))) + u8::from(File::from(sq)) + 36);

        let square = i16::from(to_16x12(self));
        let square = *FROM_16X12.get(usize::from(square.wrapping_add(direction.to_16x12().into()) as u16))?;

        unsafe {
            Some(Self::from_u8_unchecked(square?))
        }
    }

    pub fn north(self) -> Option<Self> {
        self.travel(Direction::North)
    }

    pub fn north_east(self) -> Option<Self> {
        self.travel(Direction::NorthEast)
    }

    pub fn east(self) -> Option<Self> {
        self.travel(Direction::East)
    }

    pub fn south_east(self) -> Option<Self> {
        self.travel(Direction::SouthEast)
    }

    pub fn south(self) -> Option<Self> {
        self.travel(Direction::South)
    }

    pub fn south_west(self) -> Option<Self> {
        self.travel(Direction::SouthWest)
    }

    pub fn west(self) -> Option<Self> {
        self.travel(Direction::West)
    }

    pub fn north_west(self) -> Option<Self> {
        self.travel(Direction::NorthWest)
    }

    /// The colour-dependent north of a square.
    pub fn relative_north(self, colour: Colour) -> Option<Self> {
        match colour {
            Colour::White => self.north(),
            Colour::Black => self.south(),
        }
    }

    /// The colour-dependent south of a square.
    pub fn relative_south(self, colour: Colour) -> Option<Self> {
        match colour {
            Colour::White => self.south(),
            Colour::Black => self.north(),
        }
    }

    /// An iterator over the squares a pawn attacks.
    pub fn pawn_attacks(self, colour: Colour) -> PawnIter {
        let relative_north = match colour {
            Colour::White => self.north(),
            Colour::Black => self.south(),
        };

        PawnIter(relative_north, 0)
    }

    /// An iterator over the squares a knight attacks.
    pub const fn knight_attacks(self) -> KnightIter {
        KnightIter(self, 0)
    }

    /// An iterator over the squares a king attacks.
    pub const fn king_attacks(self) -> KingIter {
        KingIter(self, 0)
    }

    /// An iterator over the squares in a `Direction`.
    pub const fn ray_attacks(self, dir: Direction) -> RayIter {
        RayIter(self, dir)
    }
}

/// A chess direction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    /// North.
    North,
    /// Northeast.
    NorthEast,
    /// East.
    East,
    /// Southeast.
    SouthEast,
    /// South.
    South,
    /// Southwest.
    SouthWest,
    /// West.
    West,
    /// Northwest.
    NorthWest,
}

impl Direction {
    /// The `Direction` 180 degrees of the given `Direction`.
    pub fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::East => Self::West,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::SouthEast,
        }
    }

    /// Returns true if the direction is diagonal.
    pub fn diagonal(self) -> bool {
        match self {
            Self::NorthEast | Self::SouthEast | Self::SouthWest | Self::NorthWest => true,
            Self::North | Self::East | Self::South | Self::West => false,
        }
    }

    /// Returns the 16x12 square difference of this Direction.
    pub fn to_16x12(self) -> i8 {
        match self {
            Direction::North => 16,
            Direction::NorthEast => 17,
            Direction::East => 1,
            Direction::SouthEast => -15,
            Direction::South => -16,
            Direction::SouthWest => -17,
            Direction::West => -1,
            Direction::NorthWest => 15,
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
        let north2 = self.0.north().and_then(Square::north);
        let east2 = self.0.east().and_then(Square::east);
        let south2 = self.0.south().and_then(Square::south);
        let west2 = self.0.west().and_then(Square::west);

        loop {
            let next = match self.1 {
                0 => north2.and_then(Square::east),
                1 => east2.and_then(Square::north),
                2 => east2.and_then(Square::south),
                3 => south2.and_then(Square::east),
                4 => south2.and_then(Square::west),
                5 => west2.and_then(Square::south),
                6 => west2.and_then(Square::north),
                7 => north2.and_then(Square::west),
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
