use std::convert::TryFrom;
use std::array::TryFromSliceError;
use std::str::FromStr;
use std::fmt;

use crate::chunk;

#[derive(Debug, Eq, PartialEq)]
struct ChunkType {
    value: [u8; 4]
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.value
    }

    fn is_valid(&self) -> bool {
        let valid_types = ["IHDR", "PLTE", "IDAT", "IEND", "tEXt", "zTXt", "iTXt", "pHYs"].map(|s| ChunkType::from_str(s).unwrap());
        valid_types.contains(self)
    }

    fn is_critical(&self) -> bool {
        self.value[0] & (1 << 5) == 0
    }

    fn is_public(&self) -> bool {
        self.value[1] & (1 << 5) == 0
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.value[2] & (1 << 5) == 0
    }

    fn is_safe_to_copy(&self) -> bool {
        self.value[3] & (1 << 5) != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ();

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { value: bytes })
    }
}

#[derive(Debug)]
enum ChunkTypeError {
    SizeError(TryFromSliceError),
    InvalidChunkType
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let chunk = ChunkType { value: s.as_bytes().try_into().map_err(ChunkTypeError::SizeError)? };
        // if chunk.is_valid() {
        //     Ok(chunk)
        // } else {
        //     Err(ChunkTypeError::InvalidChunkType)
        // }
        Ok(ChunkType { value: s.as_bytes().try_into().map_err(ChunkTypeError::SizeError)? })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

