#![allow(dead_code)]

use std::fmt::{Display, Formatter};

use blorb_chunk_types::BlorbChunkType;
use blorb_reader::BlorbReader;
use ulx_reader::UlxReader;

pub mod blorb_chunk_types;
pub mod blorb_reader;
pub mod ulx_reader;

pub enum GameType<'a> {
    Ulx(UlxReader<'a>),
    Blorb(BlorbReader<'a>),
}

impl<'a> TryFrom<&'a [u8]> for GameType<'a> {
    type Error = FileReadError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if let Ok(ulx) = value.try_into() {
            Ok(GameType::Ulx(ulx))
        } else if let Ok(blorb) = value.try_into() {
            Ok(GameType::Blorb(blorb))
        } else {
            Err(FileReadError::UnknownFileType)
        }
    }
}

impl<'a> GameType<'a> {
    pub fn get_exec(&'a self) -> UlxReader<'a> {
        match self {
            GameType::Ulx(ulx) => *ulx,
            GameType::Blorb(blorb) => blorb.get_exec(0).unwrap(),
        }
    }
}

pub(crate) fn read_be_u32(input: &[u8]) -> u32 {
    u32::from_be_bytes(input[0..4].try_into().unwrap())
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum FileReadError {
    UnexpectedStartingIdentifier(BlorbChunkType),
    /// An invalid length was supplied. Actual and Expected.
    InvalidLength(usize, usize),
    UnknownIdentifier(usize),
    InvalidConversion,
    UnknownFileType,
    UnsupportedOperation,
}

impl Display for FileReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileReadError::UnexpectedStartingIdentifier(expected) => {
                write!(f, "Unexpected starting identifier. Expected {:?}", expected)
            }
            FileReadError::InvalidLength(actual, expected) => {
                write!(
                    f,
                    "An invalid length was supplied. Actual length: {}, expected length: {}",
                    actual, expected
                )
            }
            FileReadError::UnknownIdentifier(id) => {
                write!(f, "An unknown identifier was supplied: {}", id)
            }
            FileReadError::InvalidConversion => {
                write!(f, "An invalid conversion was attempted")
            }
            FileReadError::UnknownFileType => {
                write!(f, "A file was supplied but did not fit a known file type")
            }
            FileReadError::UnsupportedOperation => {
                write!(f, "An unsupported chunk type tried to be read")
            }
        }
    }
}

impl std::error::Error for FileReadError {}
