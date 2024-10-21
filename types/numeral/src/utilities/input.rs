use crate::Input;

// Function to create Input without needing .into()
pub fn from<T>(value: T) -> Input
where
    T: Into<Input>,
{
    value.into()
}
