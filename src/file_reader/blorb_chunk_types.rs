use std::convert::TryFrom;
use std::hash::Hash;

use crate::file_reader::FileReadError;
use BlorbChunkType::*;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum BlorbChunkType {
    //Form IDs--------------------------------------------------------------------------------------
    FORM = 0x464f524d,
    //FORM
    IFRS = 0x49465253,
    RESOURCE_INDEX = 0x52496478,
    //RIdx
    PICTURE = 0x50696374,
    //Pict
    SOUND = 0x536e6420,
    //Snd
    EXECUTABLE = 0x45786563,
    //Exec
    DATA = 0x44617461,
    //Data
    //Chunk IDs-------------------------------------------------------------------------------------
    PICTURE_PNG = 0x504E4720,
    //PNG
    PICTURE_JPEG = 0x4a504547,
    //JPEG
    //	AIFF, //The chunk is a FORM type with an AIFF chunk inside
    SOUND_MOD = 0x4d4f4420,
    //MOD
    SOUND_SONG = 0x534f4e47,
    //SONG
    EXEC_ZCOD = 0x5a434f44,
    //ZCOD
    EXEC_GLUL = 0x474c554c,
    //GLUL
    TEXT = 0x54455854,
    //TEXT
    //Optional--------------------------------------------------------------------------------------
    COLOR_PALETTE = 0x506c7465,
    //Plte
    RESOLUTION = 0x5265736f,
    //Reso
    LOOP = 0x4c6f6f70,
    //Loop
    RELEASE_NUMBER = 0x52656c4e,
    //RelN
    IF_HEADER = 0x49466864,
    //IFhd
    //Optional, in many IFF FORMs-------------------------------------------------------------------
    AUTHOR = 0x41555448,
    //AUTH
    COPYRIGHT = 0x2863295f,
    //(c)_
    ANNOTATION = 0x414e4e4f, //ANNO

    INFO = 0x496E666F,
}

impl TryFrom<u32> for BlorbChunkType {
    type Error = FileReadError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x49466864 => Ok(IF_HEADER),
            0x464f524d => Ok(FORM),
            0x49465253 => Ok(IFRS),
            0x52496478 => Ok(RESOURCE_INDEX),
            0x50696374 => Ok(PICTURE),
            0x536e6420 => Ok(SOUND),
            0x45786563 => Ok(EXECUTABLE),
            0x44617461 => Ok(DATA),
            0x506c7465 => Ok(COLOR_PALETTE),
            0x5265736f => Ok(RESOLUTION),
            0x4c6f6f70 => Ok(LOOP),
            0x52656c4e => Ok(RELEASE_NUMBER),
            0x504E4720 => Ok(PICTURE_PNG),
            0x4a504547 => Ok(PICTURE_JPEG),
            0x4d4f4420 => Ok(SOUND_MOD),
            0x534f4e47 => Ok(SOUND_SONG),
            0x5a434f44 => Ok(EXEC_ZCOD),
            0x474c554c => Ok(EXEC_GLUL),
            0x496E666F => Ok(INFO),
            _ => Err(FileReadError::UnknownIdentifier(value as usize)),
        }
    }
}
