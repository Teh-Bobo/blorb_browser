#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StringTypes {
    CStyle = 0xE0,
    Compressed,
    CStyleUnicode,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParsingErrors {
    UnknownValue,
}

impl TryFrom<u8> for StringTypes {
    type Error = ParsingErrors;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use StringTypes::*;
        match value {
            0xE0 => Ok(CStyle),
            0xE1 => Ok(Compressed),
            0xE2 => Ok(CStyleUnicode),
            _ => Err(ParsingErrors::UnknownValue),
        }
    }
}

impl StringTypes {
    pub fn parse(&self, data: &[u8]) -> String {
        match self {
            StringTypes::CStyle => String::from_utf8(
                data.split(|&b| b == 0)
                    .next()
                    .expect("Never found '0' byte while parsing C-Style string.")
                    .to_vec(),
            )
            .unwrap_or("".to_string()),
            StringTypes::Compressed => "".to_string(),
            StringTypes::CStyleUnicode => "".to_string(),
        }
    }
}
