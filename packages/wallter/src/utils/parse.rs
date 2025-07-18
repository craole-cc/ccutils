use crate::prelude::{
  Error::Parse, Result, TIMESTAMP_HEADER_BYTES, TIMESTAMP_PREFIX_BYTES,
  TIMESTAMP_SIZE, TIMESTAMP_SUFFIX_BYTES
};
use chrono::NaiveTime;

/// Errors that can occur when deserializing a `NightlightSettings` struct from
/// a byte slice.
#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Invalid struct header: expected {expected:02X?}, got {actual:02X?}")]
  StructHeader { expected: Vec<u8>, actual: Vec<u8> },
  #[error("Invalid struct footer")]
  StructFooter,
  #[error("Invalid timestamp block")]
  TimestampBlock,
  #[error("Invalid array conversion")]
  SliceArrayConversion,
  #[error("Invalid block '{0}'")]
  Block(String),
  #[error("Invalid time value")]
  TimeValue
}

/// Converts a time block's hour and minute values to a `NaiveTime`.
pub fn time_to_naive_time(hours: u8, minutes: u8) -> Result<NaiveTime> {
  NaiveTime::from_hms_opt(u32::from(hours), u32::from(minutes), 0)
    .ok_or(Parse(Error::TimeValue))
}

/// Converts a Unix timestamp to a 5-byte array using a variable-length encoding
/// scheme. See `NightlightSettings`` for more information about the binary
/// format.
///
/// This custom encoding uses the most significant bit (MSB) of the first four
/// bytes as a continuation flag (set to 1, i.e., 0x80). The actual 7 bits of
/// data are stored in the lower 7 bits of these bytes. The last byte (index 4)
/// contains the most significant bits of the timestamp and does not have the
/// continuation flag set.
pub fn timestamp_to_bytes(timestamp: u64) -> [u8; TIMESTAMP_SIZE] {
  let mut bytes: [u8; TIMESTAMP_SIZE] = [0; TIMESTAMP_SIZE];
  bytes[0] = (timestamp & 0x7F | 0x80) as u8;
  bytes[1] = ((timestamp >> 7) & 0x7F | 0x80) as u8;
  bytes[2] = ((timestamp >> 14) & 0x7F | 0x80) as u8;
  bytes[3] = ((timestamp >> 21) & 0x7F | 0x80) as u8;
  bytes[4] = (timestamp >> 28) as u8;
  bytes
}

/// Converts a 5-byte array to a Unix timestamp using a variable-length decoding
/// scheme. See `NightlightSettings`` for more information about the binary
/// format.
///
/// This function decodes a timestamp from a 5-byte array, reversing the custom
/// encoding applied by `timestamp_to_bytes`. It extracts 7 bits from the first
/// four bytes (by masking out the MSB) and the remaining bits from the last
/// byte.
pub fn timestamp_from_bytes(bytes: [u8; TIMESTAMP_SIZE]) -> u64 {
  let mut timestamp: u64 = 0;
  timestamp |= (bytes[4] as u64) << 28;
  timestamp |= ((bytes[3] & 0x7F) as u64) << 21;
  timestamp |= ((bytes[2] & 0x7F) as u64) << 14;
  timestamp |= ((bytes[1] & 0x7F) as u64) << 7;
  timestamp |= (bytes[0] & 0x7F) as u64;
  timestamp
}

/// Converts a color temperature in Kelvin to a 2-byte array using a mangled
/// encoding scheme.
///
/// This custom encoding packs a 16-bit color temperature into two bytes.
/// The lower 6 bits of `color_temperature` are transformed by multiplying by 2
/// and adding 0x80, then stored in `bytes[0]`. The remaining higher 10 bits
/// are stored directly in `bytes[1]`.
pub fn kelvin_to_bytes(color_temperature: u16) -> [u8; 2] {
  let mut bytes: [u8; 2] = [0; 2];
  bytes[0] = ((color_temperature & 0x3F) * 2 + 0x80) as u8;
  bytes[1] = (color_temperature >> 6) as u8;
  bytes
}

/// Converts a 2-byte array to a color temperature in Kelvin using a mangled
/// decoding scheme.
///
/// This function decodes a color temperature from a 2-byte array, reversing
/// the custom encoding applied by `kelvin_to_bytes`. It reconstructs the 16-bit
/// value by first shifting the bits from `bytes[1]` and then reversing the
/// transformation on `bytes[0]` to get the lower 6 bits.
pub fn kelvin_from_bytes(bytes: [u8; 2]) -> u16 {
  let mut kelvin: u16 = 0;
  kelvin |= (bytes[1] as u16) << 6;
  kelvin |= ((bytes[0] - 0x80) / 2) as u16;
  kelvin
}

/// Parses the last-modified timestamp block.
pub fn last_modified_timestamp_block(
  data: &[u8],
  start_from: usize
) -> Result<(u64, usize)> {
  let mut pos: usize = start_from;
  // Check timestamp header bytes
  if data[pos..pos + TIMESTAMP_HEADER_BYTES.len()] != TIMESTAMP_HEADER_BYTES {
    return Err(Parse(Error::TimestampBlock));
  }
  pos += TIMESTAMP_HEADER_BYTES.len();
  // Check timestamp prefix bytes
  if data[pos..pos + TIMESTAMP_PREFIX_BYTES.len()] != TIMESTAMP_PREFIX_BYTES {
    return Err(Parse(Error::TimestampBlock));
  }
  pos += TIMESTAMP_PREFIX_BYTES.len();

  // Parse timestamp from bytes
  let timestamp_slice: [u8; TIMESTAMP_SIZE] = data[pos..pos + TIMESTAMP_SIZE]
    .try_into()
    .map_err(|_| Error::SliceArrayConversion)?;
  pos += TIMESTAMP_SIZE;
  let timestamp = timestamp_from_bytes(timestamp_slice);

  // Check timestamp suffix bytes
  if data[pos..pos + TIMESTAMP_SUFFIX_BYTES.len()] != TIMESTAMP_SUFFIX_BYTES {
    return Err(Parse(Error::TimestampBlock));
  }
  pos += TIMESTAMP_SUFFIX_BYTES.len();

  Ok((timestamp, pos))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_timestamp_roundtrip_conversion() {
    let timestamp = 1742518000;
    let bytes = timestamp_to_bytes(timestamp);
    let timestamp_from_bytes = timestamp_from_bytes(bytes);
    assert_eq!(timestamp, timestamp_from_bytes);
  }

  #[test]
  fn test_kelvin_roundtrip_conversion() {
    let color_temperature = 2700;
    let bytes = kelvin_to_bytes(color_temperature);
    let kelvin_from_bytes = kelvin_from_bytes(bytes);
    assert_eq!(color_temperature, kelvin_from_bytes);
  }
}
