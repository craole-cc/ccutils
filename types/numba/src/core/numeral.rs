#[derive(Default, Debug, Clone, PartialEq)]
pub struct Numeral {
	pub value: astro_float::BigFloat,
	// pub rounding: Option<crate::Rounding>,
	// pub cardinal: crate::Cardinal,
	// pub ordinal: crate::Ordinal,
	// pub percentage: crate::Percentage,
	// pub fraction: crate::Fraction,
	// pub roman: crate::Roman,
}
