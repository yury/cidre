pub type FourCharCode = u32;

pub type ResType = FourCharCode; // ??
pub type UniChar = u16;

pub fn four_cc_to_string(bytes: [u8; 4]) -> String {
    let mut bytes = bytes;
    four_cc_to_str(&mut bytes).to_owned()
}

pub fn four_cc_to_str(bytes: &mut [u8; 4]) -> &str {
    for b in bytes.iter_mut() {
        if *b < b' ' || *b > b'~' {
            *b = b'.';
        }
    }
    // SAFETY: All bytes are in the ASCII range, so the result is valid UTF-8.
    unsafe { std::str::from_utf8_unchecked(bytes) }
}

pub fn four_cc_fmt_debug(
    val: u32,
    name: &str,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    let mut fcc = val.to_be_bytes();
    f.debug_struct(name)
        .field("raw", &val)
        .field("fcc", &four_cc_to_str(&mut fcc))
        .finish()
}

#[cfg(test)]
mod tests {
    use crate::four_cc_to_str;

    #[test]
    fn basics() {
        let mut x: [u8; 4] = (*b"24BG").into();
        let s = four_cc_to_str(&mut x);
        assert_eq!(s, "24BG");

        let mut bytes = 0u32.to_be_bytes();
        let s = four_cc_to_str(&mut bytes);
        assert_eq!(s, "....");
    }
}
