use std::borrow::Cow;
use crate::ToStringRef;

macro_rules! impl_to_string_ref_numeric {
    ($($t:ty),*) => {
        $(
            impl ToStringRef for $t {
                #[inline]
                fn to_string_ref(&self) -> Cow<'_, str> {
                    // For integers and small floats, we can pre-allocate
                    // a reasonable buffer size
                    let mut buffer = String::with_capacity(20);
                    use std::fmt::Write;
                    write!(buffer, "{}", self).unwrap();
                    Cow::Owned(buffer)
                }
            }
        )*
    }
}

impl_to_string_ref_numeric!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
);
