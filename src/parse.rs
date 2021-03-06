use crate::{result, Error};

pub fn i32_to_usize(value: i32) -> result::Result<usize> {
    usize::try_from(value).map_err(|_| Error::IntConversion)
}

pub fn usize_to_i32(value: usize) -> result::Result<i32> {
    i32::try_from(value).map_err(|_| Error::IntConversion)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_i32_to_usize() {
        let parsed_value = i32_to_usize(42).expect("failed to parse i32");
        assert_eq!(parsed_value, 42);
    }

    #[test]
    fn parse_usize_to_i32() {
        let parsed_value = usize_to_i32(81).expect("failed to parse usize");
        assert_eq!(parsed_value, 81);
    }
}
