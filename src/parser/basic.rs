use super::ParseGDIIRes;
use nom::bytes::streaming::take;

// return valid data size(exclude two byte "size" and two byte "type")
pub(super) fn take_size(s: &str) -> ParseGDIIRes<&str, usize> {
    let (s, d) = take(2usize)(s)?;
    let parsed_d = usize::from_str_radix(d, 16).unwrap();
    Ok((s, parsed_d - 4usize))
}

pub(super) fn take_type(s: &str) -> ParseGDIIRes<&str, &str> {
    take(2usize)(s)
}
