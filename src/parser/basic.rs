use super::ParseGDIIRes;
use byteorder::{BigEndian, ByteOrder};
use nom::bytes::streaming::{tag, take};

// return valid data size(exclude two byte "size" and two byte "type")
pub(super) fn take_size(s: &[u8]) -> ParseGDIIRes<&[u8], usize> {
    let (s, d) = take(2usize)(s)?;
    let parsed_d = BigEndian::read_u16(&d);
    Ok((s, parsed_d as usize))
}

pub(super) fn take_type(s: &[u8]) -> ParseGDIIRes<&[u8], [u8; 2]> {
    let (s, d) = take(2usize)(s)?;
    Ok((s, [d[0], d[1]]))
}

pub(super) fn end_tag(s: &[u8]) -> ParseGDIIRes<&[u8], &[u8]> {
    tag([0x00, 0x04, 0x04, 0x00])(s)
}
