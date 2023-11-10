use super::BlorbChunkType::EXEC_GLUL;
use super::FileReadError::UnexpectedStartingIdentifier;
use super::{read_be_u32, BlorbChunkType, FileReadError};
use crate::strings::StringTypes;
use std::fmt::{Debug, Display, Formatter};
use TryInto;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UlxReader<'a> {
    pub header: GlulxHeader,
    pub debugging_header: GlulxDebuggingHeader,
    pub memory: &'a [u8],
}

impl<'a> TryFrom<&'a [u8]> for UlxReader<'a> {
    type Error = FileReadError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let header: GlulxHeader = value.try_into()?;
        let debugging_header: GlulxDebuggingHeader = value[HEADER_SIZE..].try_into()?;
        let memory = value;
        Ok(UlxReader {
            header,
            debugging_header,
            memory,
        })
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) struct ParsedString {
    pub(crate) data: String,
    pub(crate) start_address: usize,
    pub(crate) string_type: StringTypes,
}

impl<'a> UlxReader<'a> {
    pub(crate) fn parse_strings(&self) -> Vec<ParsedString> {
        let mut ret = Vec::new();
        for i in 0..self.memory.len() {
            if let Ok(string_type) = StringTypes::try_from(self.memory[i]) {
                let data = string_type.parse(&self.memory[i + 1..]);
                if !data.is_empty() {
                    ret.push(ParsedString {
                        data,
                        start_address: i,
                        string_type,
                    })
                }
            }
        }
        ret.iter().for_each(|ps| println!("{ps:?}"));
        ret
    }
}

// The size of the GlulxHeader in bytes
static HEADER_SIZE: usize = 36;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GlulxHeader {
    pub magic_num: u32,
    pub version: u32,
    pub ram_start: u32,
    pub ext_start: u32,
    pub end_mem: u32,
    pub stack_size: u32,
    pub start_function_address: u32,
    pub decoding_table_address: u32,
    pub checksum: u32,
}

impl GlulxHeader {
    pub fn get_major_version(&self) -> u16 {
        (self.version >> 16) as u16
    }

    pub fn get_minor_version(&self) -> u8 {
        ((self.version & 0xff00) >> 8) as u8
    }

    pub fn get_sub_minor_version(&self) -> u8 {
        self.version as u8
    }
}

impl Display for GlulxHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "GlulxHeader {{ magic_num: {}, version: {}.{}.{}, ram_start: {}, ext_start: {}, \
            end_mem: {}, stack_size: {}, start_function_address: {}, decoding_table_address: {}, \
            checksum: {} }}",
            String::from_utf8_lossy(&self.magic_num.to_be_bytes()),
            self.get_major_version(),
            self.get_minor_version(),
            self.get_sub_minor_version(),
            self.ram_start,
            self.ext_start,
            self.end_mem,
            self.stack_size,
            self.start_function_address,
            self.decoding_table_address,
            self.checksum
        ))
    }
}

static GLUL_AS_NUM: u32 = 1198290284;
impl TryFrom<&[u8]> for GlulxHeader {
    type Error = FileReadError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let magic_num = read_be_u32(&bytes[..4]);
        if magic_num != GLUL_AS_NUM {
            return Err(UnexpectedStartingIdentifier(EXEC_GLUL));
        }
        Ok(GlulxHeader {
            magic_num,
            version: read_be_u32(&bytes[4..8]),
            ram_start: read_be_u32(&bytes[8..12]),
            ext_start: read_be_u32(&bytes[12..16]),
            end_mem: read_be_u32(&bytes[16..20]),
            stack_size: read_be_u32(&bytes[20..24]),
            start_function_address: read_be_u32(&bytes[24..28]),
            decoding_table_address: read_be_u32(&bytes[28..32]),
            checksum: read_be_u32(&bytes[32..36]),
        })
    }
}

// The size of the debugging header in bytes
#[allow(unused)]
static DEBUGGING_HEADER_SIZE: usize = 24;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GlulxDebuggingHeader {
    pub id: u32,
    pub memory_layout: u32,
    pub inform_version: u32,
    pub glulx_compiler_version: u32,
    pub game_version: u16,
    pub game_serial_number: [u8; 6],
}

impl Display for GlulxDebuggingHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "GlulDebuggingHeader {{ id: {}, memory_layout: {}, \
        inform_version: {}, glulx_compiler_version: {}, game_version: {}, game_serial_number: {:?} (\"{}\") \
        }}",
            String::from_utf8_lossy(&self.id.to_be_bytes()),
            self.memory_layout,
            String::from_utf8_lossy(&self.inform_version.to_be_bytes()),
            String::from_utf8_lossy(&self.glulx_compiler_version.to_be_bytes()),
            self.game_version,
            self.game_serial_number,
            String::from_utf8_lossy(&self.game_serial_number)
        ))
    }
}

static INFO_AS_NUM: u32 = 1231971951;

impl TryFrom<&[u8]> for GlulxDebuggingHeader {
    type Error = FileReadError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let id = read_be_u32(&value[..4]);
        if id != INFO_AS_NUM {
            return Err(UnexpectedStartingIdentifier(BlorbChunkType::INFO));
        }
        Ok(GlulxDebuggingHeader {
            id,
            memory_layout: read_be_u32(&value[4..8]),
            inform_version: read_be_u32(&value[8..12]),
            glulx_compiler_version: read_be_u32(&value[12..16]),
            game_version: u16::from_be_bytes(value[16..18].try_into().unwrap()),
            game_serial_number: value[18..24].try_into().unwrap(),
        })
    }
}
