use crate::{Error, Result, consts::*, utils::parse};
use std::{
  io,
  time::{SystemTime, UNIX_EPOCH}
};
use winreg::{RegKey, enums::*};

const NIGHTLIGHT_STATE_REGISTRY_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\CloudStore\Store\DefaultAccount\Current\default$windows.data.bluelightreduction.bluelightreductionstate\windows.data.bluelightreduction.bluelightreductionstate";
const NIGHTLIGHT_STATE_REGISTRY_VAL: &str = "Data";
const NIGHTLIGHT_STATE_ENABLED_BYTES: [u8; 2] = [0x10, 0x00];

/// The nightlight state data structure has the following binary format:
///
/// * [STRUCT_HEADER_BYTES]
/// * [TIMESTAMP_HEADER_BYTES]
/// * [TIMESTAMP_PREFIX_BYTES]
/// * The last-modified Unix timestamp in seconds, variably-encoded into
///   [TIMESTAMP_SIZE] bytes
///     - byte 0: bits 0-6 = timestamp's bits 0-6, but top bit 7 is always set
///     - byte 1: bits 0-6 = timestamp's bits 7-13, but top bit 7 is always set
///     - byte 2: bits 0-6 = timestamp's bits 14-20, but top bit 7 is always set
///     - byte 3: bits 0-6 = timestamp's bits 21-27, but top bit 7 is always set
///     - byte 4: bits 0-6 = timestamp's bits 28-31, but top bit 7 is NOT set
/// * [TIMESTAMP_SUFFIX_BYTES]
/// * A single byte indicating the size of the following data block.
///     - The purpose of these remaining bytes is currently unknown. Known
///       values for the size byte are:
///         - 0x13 (19 bytes) when `is_enabled` is false.
///         - 0x15 (21 bytes) when `is_enabled` is true.
/// * [STRUCT_HEADER_BYTES] again
/// * If `is_enabled` is true, [NIGHTLIGHT_STATE_ENABLED_BYTES] will be present.
/// * A block of unknown bytes that change over time.
/// * [STRUCT_FOOTER_BYTES]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
  /// The last-modified Unix timestamp in seconds
  pub timestamp: u64,
  /// Whether the nightlight is (force) enabled or not.
  /// If true, then the nightlight will be enabled regardless of the schedule
  /// settings.
  pub is_enabled: bool,
  /// The remaining data bytes read from the registry
  remaining_data: Vec<u8>
}

impl State {
  /// Helper to open the nightlight registry key with specific access and error
  /// handling. This centralizes the common logic for both reading and writing
  /// to the registry.
  fn open_nightlight_registry_key(access: u32) -> Result<RegKey> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu
      .open_subkey_with_flags(NIGHTLIGHT_STATE_REGISTRY_KEY, access)
      .map_err(|e| {
        let error_kind = if access == KEY_READ {
          io::ErrorKind::NotFound // Common for read failures if key doesn't exist
        } else {
          io::ErrorKind::PermissionDenied // Common for write failures due to permissions
        };
        Error::IO(io::Error::new(
          error_kind,
          format!(
            "Failed to open registry key '{NIGHTLIGHT_STATE_REGISTRY_KEY}' with access {access}: {e}"
          )
        ))
      })
  }

  /// Reads the nightlight state from the Windows registry
  pub fn read_from_registry() -> Result<Self> {
    let key = Self::open_nightlight_registry_key(KEY_READ).map_err(|e| {
      Error::IO(io::Error::new(
        io::ErrorKind::NotFound,
        format!(
          "Failed to open registry key '{NIGHTLIGHT_STATE_REGISTRY_KEY}': {e}"
        )
      ))
    })?;

    // Read raw bytes from registry - we need to use get_raw_value for binary
    // data
    let reg_value = key.get_raw_value(NIGHTLIGHT_STATE_REGISTRY_VAL).map_err(|e| {
        Error::IO(io::Error::new(
          io::ErrorKind::NotFound, // Specific to reading the value
          format!("Failed to read registry value '{NIGHTLIGHT_STATE_REGISTRY_VAL}': {e}"),
        ))
      })?;

    let data = reg_value.bytes;
    Self::deserialize_from_bytes(&data)
  }

  /// Writes the nightlight state to the Windows registry
  pub fn write_to_registry(&self) -> Result<()> {
    let key = Self::open_nightlight_registry_key(KEY_SET_VALUE)?;

    let data = self.serialize_to_bytes();

    // Set raw binary data to registry
    key
      .set_raw_value(
        NIGHTLIGHT_STATE_REGISTRY_VAL,
        &winreg::RegValue {
          bytes: data,
          vtype: winreg::enums::RegType::REG_BINARY
        }
      )
      .map_err(|e| {
        Error::IO(io::Error::new(
          io::ErrorKind::PermissionDenied, // Specific to writing the value
          format!("Failed to write registry value '{NIGHTLIGHT_STATE_REGISTRY_VAL}': {e}"),
        ))
      })?;

    Ok(())
  }

  /// Deserializes a [State] struct from a byte slice.
  /// See [State] for more information about the binary format.
  pub fn deserialize_from_bytes(data: &[u8]) -> Result<Self> {
    let mut pos = 0;
    let end = pos + STRUCT_HEADER_BYTES.len();
    if data.get(pos..end) != Some(&STRUCT_HEADER_BYTES) {
      return Err(Error::Parse(parse::Error::StructHeader {
        expected: STRUCT_HEADER_BYTES.to_vec(),
        actual: data.get(pos..end).unwrap_or_default().to_vec()
      }));
    }
    pos = end;

    // Use parse::last_modified_timestamp_block for timestamp parsing
    let (timestamp, new_pos) = parse::last_modified_timestamp_block(data, pos)?;
    pos = new_pos;

    // Read the byte indicating the length of the subsequent block (including
    // itself)
    let remaining_struct_size_byte_value = *data.get(pos).ok_or_else(|| {
      Error::Parse(parse::Error::Block("Missing struct size byte".to_string()))
    })? as usize;
    pos += 1; // Consume the size byte

    // The `remaining_struct_size_byte_value` includes the size byte itself.
    // So, the actual content length after the size byte is
    // `remaining_struct_size_byte_value - 1`. The total length of the data
    // from the current `pos` to the end of the slice
    // should be `(remaining_struct_size_byte_value - 1) +
    // STRUCT_FOOTER_BYTES.len()`.
    let expected_remaining_data_len =
      (remaining_struct_size_byte_value - 1) + STRUCT_FOOTER_BYTES.len();
    if data.len() - pos != expected_remaining_data_len {
      return Err(Error::Parse(parse::Error::Block(format!(
        "Invalid struct size: expected {} bytes from pos {}, got {} bytes total. Size byte value: {}",
        expected_remaining_data_len,
        pos,
        data.len() - pos,
        remaining_struct_size_byte_value
      ))));
    }

    let end = pos + STRUCT_HEADER_BYTES.len();
    if data.get(pos..end) != Some(&STRUCT_HEADER_BYTES) {
      return Err(Error::Parse(parse::Error::StructHeader {
        expected: STRUCT_HEADER_BYTES.to_vec(),
        actual: data.get(pos..end).unwrap_or_default().to_vec()
      }));
    }
    pos = end;

    let (is_enabled, new_pos) = if data
      .get(pos..pos + NIGHTLIGHT_STATE_ENABLED_BYTES.len())
      == Some(&NIGHTLIGHT_STATE_ENABLED_BYTES)
    {
      (true, pos + NIGHTLIGHT_STATE_ENABLED_BYTES.len())
    } else {
      (false, pos)
    };
    pos = new_pos;

    // Read the remaining data bytes and save it if we need to write it back
    let end_of_remaining_data =
      data.len().saturating_sub(STRUCT_FOOTER_BYTES.len());
    let remaining_data_slice =
      data.get(pos..end_of_remaining_data).ok_or_else(|| {
        Error::Parse(parse::Error::Block(
          "Invalid remaining data slice".to_string()
        ))
      })?;
    let remaining_data_vec = Vec::from(remaining_data_slice);
    pos += remaining_data_vec.len();

    let end = pos + STRUCT_FOOTER_BYTES.len();
    if data.get(pos..end) != Some(&STRUCT_FOOTER_BYTES) {
      return Err(Error::Parse(parse::Error::StructFooter));
    }
    pos = end;

    println!(
      "[DEBUG] State::deserialize_from_bytes: Parsed state: timestamp={timestamp}, is_enabled={is_enabled}"
    );
    Ok(Self {
      timestamp,
      is_enabled,
      remaining_data: remaining_data_vec
    })
  }

  /// Serializes a [State] struct into a byte slice.
  /// See [State] for more information about the binary format.
  pub fn serialize_to_bytes(&self) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    println!("[DEBUG] State::serialize_to_bytes: Serializing state: {self:?}");

    bytes.extend_from_slice(&STRUCT_HEADER_BYTES);
    bytes.extend_from_slice(&TIMESTAMP_HEADER_BYTES);
    bytes.extend_from_slice(&TIMESTAMP_PREFIX_BYTES);
    let timestamp_bytes = parse::timestamp_to_bytes(self.timestamp);
    bytes.extend_from_slice(&timestamp_bytes);
    bytes.extend_from_slice(&TIMESTAMP_SUFFIX_BYTES);

    // Figure out the size of the remaining bytes after computing the back bytes
    let mut remaining_struct_bytes_content: Vec<u8> = Vec::new();
    remaining_struct_bytes_content.extend_from_slice(&STRUCT_HEADER_BYTES);

    if self.is_enabled {
      remaining_struct_bytes_content
        .extend_from_slice(&NIGHTLIGHT_STATE_ENABLED_BYTES);
    }
    remaining_struct_bytes_content.extend_from_slice(&self.remaining_data);

    // The size byte itself is included in the count, so add 1 to the content
    // length
    let remaining_struct_size_byte_value =
      (remaining_struct_bytes_content.len() + 1) as u8;
    bytes.push(remaining_struct_size_byte_value);
    bytes.extend(remaining_struct_bytes_content);
    bytes.extend_from_slice(&STRUCT_FOOTER_BYTES);
    bytes
  }

  fn update_timestamp(&mut self) {
    self.timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
  }

  /// Enables the nightlight and updates the timestamp.
  /// Returns true if a change was made (i.e. the nightlight was previously
  /// disabled).
  pub fn enable(&mut self) -> bool {
    println!(
      "[DEBUG] State::enable: Called. Current state is_enabled={}",
      self.is_enabled
    );
    if !self.is_enabled {
      self.is_enabled = true;
      self.update_timestamp();
      true
    } else {
      false
    }
  }

  /// Disables the nightlight and updates the timestamp.
  /// Returns true if a change was made (i.e. the nightlight was previously
  /// enabled).
  pub fn disable(&mut self) -> bool {
    println!(
      "[DEBUG] State::disable: Called. Current state is_enabled={}",
      self.is_enabled
    );
    if self.is_enabled {
      self.is_enabled = false;
      self.update_timestamp();
      true
    } else {
      false
    }
  }

  /// Convenience method to enable nightlight and write to registry
  pub fn enable_and_save(&mut self) -> Result<bool> {
    let changed = self.enable();
    if changed {
      self.write_to_registry()?;
    }
    Ok(changed)
  }

  /// Convenience method to disable nightlight and write to registry
  pub fn disable_and_save(&mut self) -> Result<bool> {
    let changed = self.disable();
    if changed {
      self.write_to_registry()?;
    }
    Ok(changed)
  }
}

/// Gets the current nightlight state from the registry.
pub fn get_state() -> Result<State> {
  State::read_from_registry()
}

/// Checks if nightlight is currently enabled by reading from the registry.
pub fn is_enabled() -> Result<bool> {
  Ok(get_state()?.is_enabled)
}

/// Gets the current nightlight state from the registry.
///
/// Returns `true` if the state was changed, `false` otherwise.
pub fn enable() -> Result<bool> {
  let mut state = State::read_from_registry()?;
  if !state.is_enabled {
    state.is_enabled = true;
    state.timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
    state.write_to_registry()?;
    Ok(true)
  } else {
    Ok(false)
  }
}

/// Disables nightlight and saves the state to the registry.
///
/// Returns `true` if the state was changed, `false` otherwise.
pub fn disable() -> Result<bool> {
  let mut state = State::read_from_registry()?;
  if state.is_enabled {
    state.is_enabled = false;
    state.timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
    state.write_to_registry()?;
    Ok(true)
  } else {
    Ok(false)
  }
}

/// Toggles the nightlight state and saves it to the registry.
///
/// Returns a tuple `(changed, new_state)`, where `changed` is a boolean
/// indicating if the state was modified, and `new_state` is the new boolean
/// state.
pub fn toggle() -> Result<(bool, bool)> {
  if is_enabled()? {
    disable()?;
    Ok((true, false))
  } else {
    enable()?;
    Ok((true, true))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  // Test data (no changes, just uncommenting for context)
  const BYTES_DISABLED: [u8; 41] = [
    0x43, 0x42, 0x01, 0x00, 0x0A, 0x02, 0x01, 0x00, 0x2A, 0x06, 0x89, 0x95,
    0xFC, 0xBE, 0x06, 0x2A, 0x2B, 0x0E, 0x13, 0x43, 0x42, 0x01, 0x00,
    0xD0, // size byte 19
    0x0A, 0x02, 0xC6, 0x14, 0xA9, 0xF6, 0xE2, 0xD3, 0xEF, 0xEA, 0xE6, 0xED,
    0x01, 0x00, 0x00, 0x00, 0x00 // 4-byte footer
  ];
  const BYTES_ENABLED: [u8; 43] = [
    0x43, 0x42, 0x01, 0x00, 0x0A, 0x02, 0x01, 0x00, 0x2A, 0x06, 0x89, 0x95,
    0xFC, 0xBE, 0x06, 0x2A, 0x2B, 0x0E, 0x15, 0x43, 0x42, 0x01, 0x00,
    0x10, // size byte 21
    0x00, 0xD0, 0x0A, 0x02, 0xC6, 0x14, 0xA9, 0xF6, 0xE2, 0xD3, 0xEF, 0xEA,
    0xE6, 0xED, 0x01, 0x00, 0x00, 0x00, 0x00 // 4-byte footer
  ];

  #[test]
  fn test_serialize_to_bytes() {
    let state_disabled = State {
      timestamp: 1742670473,
      is_enabled: false,
      remaining_data: vec![
        0xD0, 0x0A, 0x02, 0xC6, 0x14, 0xA9, 0xF6, 0xE2, 0xD3, 0xEF, 0xEA, 0xE6,
        0xED, 0x01,
      ]
    };
    let bytes_disabled = state_disabled.serialize_to_bytes();
    assert_eq!(bytes_disabled, BYTES_DISABLED);

    let state_enabled = State {
      timestamp: 1742670473,
      is_enabled: true,
      remaining_data: vec![
        0xD0, 0x0A, 0x02, 0xC6, 0x14, 0xA9, 0xF6, 0xE2, 0xD3, 0xEF, 0xEA, 0xE6,
        0xED, 0x01,
      ]
    };
    let bytes_enabled = state_enabled.serialize_to_bytes();
    assert_eq!(bytes_enabled, BYTES_ENABLED);
  }

  #[test]
  fn test_deserialize_from_bytes() {
    let expected_state_disabled = State {
      timestamp: 1742670473,
      is_enabled: false,
      remaining_data: vec![
        0xD0, 0x0A, 0x02, 0xC6, 0x14, 0xA9, 0xF6, 0xE2, 0xD3, 0xEF, 0xEA, 0xE6,
        0xED, 0x01,
      ]
    };
    let state_disabled =
      State::deserialize_from_bytes(&BYTES_DISABLED).unwrap();
    assert_eq!(state_disabled, expected_state_disabled);

    let expected_state_enabled = State {
      timestamp: 1742670473,
      is_enabled: true,
      remaining_data: vec![
        0xD0, 0x0A, 0x02, 0xC6, 0x14, 0xA9, 0xF6, 0xE2, 0xD3, 0xEF, 0xEA, 0xE6,
        0xED, 0x01,
      ]
    };
    let state_enabled = State::deserialize_from_bytes(&BYTES_ENABLED).unwrap();
    assert_eq!(state_enabled, expected_state_enabled);
  }

  #[test]
  fn test_serde_roundtrip() {
    let state_disabled =
      State::deserialize_from_bytes(&BYTES_DISABLED).unwrap();
    let bytes = state_disabled.serialize_to_bytes();
    let state_deserialized = State::deserialize_from_bytes(&bytes).unwrap();
    assert_eq!(state_deserialized, state_disabled);

    let state_enabled = State::deserialize_from_bytes(&BYTES_ENABLED).unwrap();
    let bytes = state_enabled.serialize_to_bytes();
    let state_deserialized = State::deserialize_from_bytes(&bytes).unwrap();
    assert_eq!(state_deserialized, state_enabled);
  }

  #[test]
  fn test_timestamp_conversion() {
    let timestamp = 1742670473u64;
    let bytes = parse::timestamp_to_bytes(timestamp); // Use parse.rs function
    let converted_back = parse::timestamp_from_bytes(bytes); // Use parse.rs function
    assert_eq!(timestamp, converted_back);
  }

  #[test]
  #[ignore] // This test modifies the registry and should be run manually.
  fn test_enable_disable() {
    // Pre-condition check: The test can only run if the Night Light registry
    // key exists, which Windows creates on first use.
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if hkcu.open_subkey(NIGHTLIGHT_STATE_REGISTRY_KEY).is_err() {
      eprintln!(
        // Keep eprintln! for test-specific output
        "Skipping test: Night Light registry key not found at '{NIGHTLIGHT_STATE_REGISTRY_KEY}'."
      );
      println!(
        "To run this test, please toggle Night Light once in Windows Settings to initialize it."
      );
      return;
    }

    // This test verifies that enable() and disable() correctly change the state
    // and report that a change was made.
    let initial_state = is_enabled()
      .expect("Failed to get initial state for enable/disable test");

    if initial_state {
      // If it's on, turn it off.
      assert!(disable().expect("Failed to disable nightlight"));
      assert!(!is_enabled().expect("State should be disabled"));
      // Turn it back on to restore the original state.
      assert!(enable().expect("Failed to re-enable nightlight"));
      assert!(is_enabled().expect("State should be enabled"));
    } else {
      // If it's off, turn it on.
      assert!(enable().expect("Failed to enable nightlight"));
      assert!(is_enabled().expect("State should be enabled"));
      // Turn it back off to restore the original state.
      assert!(disable().expect("Failed to re-disable nightlight"));
      assert!(!is_enabled().expect("State should be disabled"));
    }
  }

  #[test]
  #[ignore] // This test modifies the registry and should be run manually.
  fn test_toggle() {
    // Pre-condition check: The test can only run if the Night Light registry
    // key exists, which Windows creates on first use.
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if hkcu.open_subkey(NIGHTLIGHT_STATE_REGISTRY_KEY).is_err() {
      eprintln!(
        // Keep eprintln! for test-specific output
        "Skipping test: Night Light registry key not found at '{NIGHTLIGHT_STATE_REGISTRY_KEY}'."
      );
      println!(
        "To run this test, please toggle Night Light once in Windows Settings to initialize it."
      );
      return;
    }

    // 1. Get initial state
    let initial_state =
      is_enabled().expect("Failed to get initial state for toggle test");

    // 2. Toggle once
    let (changed1, new_state1) = toggle().expect("Failed to toggle first time");
    assert!(changed1, "Toggle should report a change the first time.");
    assert_eq!(
      new_state1, !initial_state,
      "New state should be the opposite of the initial state."
    );

    // 3. Toggle back to original state
    let (changed2, new_state2) =
      toggle().expect("Failed to toggle second time");
    assert!(changed2, "Toggle should report a change the second time.");
    assert_eq!(
      new_state2, initial_state,
      "State should be restored to the initial state after the second toggle."
    );
  }

  #[test]
  #[ignore]
  #[cfg(target_os = "windows")]
  fn test_standalone_nightlight_operations() -> Result<()> {
    // This test checks the Night Light module's public API in a standalone
    // manner. Using eprintln! for test-specific output.
    eprintln!("Starting standalone Night Light module test...");

    // Helper function for pretty printing the current status
    fn print_status(label: &str) -> bool {
      match is_enabled() {
        Ok(enabled) => {
          println!(
            "\n{}: Night Light is currently {}.",
            label, /* Keep println! for user-friendly output within test
                    * context */
            if enabled { "ON" } else { "OFF" }
          );
          true
        }
        Err(e) => {
          eprintln!("\n{label}: Failed to get Night Light status: {e}");
          if let Error::IO(io_err) = e
            && io_err.kind() == std::io::ErrorKind::NotFound
          {
            eprintln!(
              "\nHint: This error is common if Night Light has never been used on this system."
            );
            eprintln!(
              "Please toggle it on and off once in Windows Settings to create the necessary registry key."
            );
          }
          false
        }
      }
    }

    // 1. Get and print the initial state. If we can't, exit.
    if !print_status("Initial Check") {
      return Ok(());
    }

    // 2. Test enabling (set based on bool: true, or mode: Dark)
    eprintln!("\n--> Attempting to ENABLE Night Light (as for Dark Mode)...");
    match enable() {
      Ok(changed) =>
        if changed {
          println!("   [SUCCESS] State was changed. Night Light is now ON.");
        } else {
          println!(
            "   [SKIPPED] No change needed. Night Light was already ON."
          );
        },
      Err(e) => eprintln!("[ERROR] Failed to enable Night Light: {e}")
    }
    print_status("After Enable");

    // 3. Test disabling (set based on bool: false, or mode: Light)
    eprintln!("\n--> Attempting to DISABLE Night Light (as for Light Mode)...");
    match disable() {
      Ok(changed) =>
        if changed {
          println!("   [SUCCESS] State was changed. Night Light is now OFF.");
        } else {
          println!(
            "   [SKIPPED] No change needed. Night Light was already OFF."
          );
        },
      Err(e) => eprintln!("[ERROR] Failed to disable Night Light: {e}")
    }
    print_status("After Disable");

    // The toggle test from the previous setup already covers toggling
    // functionality.

    eprintln!("\nStandalone Night Light module test completed.");
    Ok(())
  }
}
