use std::simd::prelude::*;

/// Removes commas from a string using SIMD operations for efficiency.
///
/// # Arguments
///
/// * `input` - A string slice that may contain commas.
///
/// # Returns
///
/// A new `String` with all commas removed.
///
/// # Example
///
/// ```
/// let result = remove_commas("1,234,567.89");
/// assert_eq!(result, "1234567.89");
/// ```
pub fn remove_commas(input: &str) -> String {
	let bytes = input.as_bytes();
	let len = bytes.len();
	let mut result = Vec::with_capacity(len);

	// Process 32 bytes at a time
	let chunks = bytes.chunks_exact(32);
	let remainder = chunks.remainder();

	// Create a vector with all elements set to the comma byte
	let comma_vec = Simd::<u8, 32>::splat(b',');

	for chunk in chunks {
		// Load the chunk into a SIMD vector
		let v = Simd::<u8, 32>::from_slice(chunk);

		// Create a mask where the bytes are not equal to the comma byte
		let mask = v.simd_ne(comma_vec);

		// Convert mask to bitmask and process bytes
		let bitmask = mask.to_bitmask();
		for (i, &item) in chunk.iter().enumerate() {
			if (bitmask & (1 << i)) != 0 {
				result.push(item);
			}
		}
		// for i in 0..32 {
		// 	if (bitmask & (1 << i)) != 0 {
		// 		result.push(chunk[i]);
		// 	}
		// }
	}

	// Handle remaining bytes
	for &byte in remainder {
		if byte != b',' {
			result.push(byte);
		}
	}

	// Convert the result back to a String safely
	String::from_utf8(result).expect("Invalid UTF-8")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_remove_commas() {
		assert_eq!(remove_commas("1,234,567.89"), "1234567.89");
		assert_eq!(remove_commas("no,commas,left"), "nocommasleft");
		assert_eq!(remove_commas("123"), "123");
		assert_eq!(remove_commas(""), "");
	}
}
