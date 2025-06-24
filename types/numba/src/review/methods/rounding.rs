// use crate::{Direction, Rounding};

// impl Rounding {
// 	pub fn new(
// 		value: f64,
// 		direction: Direction,
// 		unit: usize,
// 	) -> Self {
// 		Self {
// 			direction,
// 			unit,
// 			precision: None,
// 		}
// 	}

// 	/// Set precision based on a fractional value.
// 	pub fn set_precision(&mut self, fractional_value: f64) {
// 		if fractional_value.fract() != 0.0 {
// 			self.precision = Some(
// 				fractional_value
// 					.to_string()
// 					.split('.')
// 					.collect::<Vec<&str>>()[1]
// 					.len(),
// 			);
// 		} else {
// 			self.precision = Some(0);
// 		}
// 	}

// 	/// Perform rounding based on the specified direction.
// 	pub fn round(&self) -> f64 {
// 		// Change return type to f64 for better precision
// 		let unit_as_f64 = self.unit as f64; // Convert usize to f64
// 		match self.direction {
// 			Direction::Up => {
// 				(self.value / unit_as_f64).ceil() * unit_as_f64
// 			}
// 			Direction::Down => {
// 				(self.value / unit_as_f64).floor() * unit_as_f64
// 			}
// 			Direction::Nearest => {
// 				let half_unit = unit_as_f64 / 2.0;
// 				if (self.value % unit_as_f64) >= half_unit {
// 					(self.value / unit_as_f64).ceil() * unit_as_f64 // Round up
// 				} else {
// 					(self.value / unit_as_f64).floor() * unit_as_f64 // Round down
// 				}
// 			}
// 		}
// 	}

// 	/// Display information about rounding.
// 	pub fn display(&self) {
// 		println!(
//             "Value: {}, Direction: {:?}, Unit: {}, Precision: {:?}, Rounded Value: {}",
//             self.value,
//             self.direction,
//             self.unit,
//             self.precision,
//             self.round()
//         );
// 	}
// }
