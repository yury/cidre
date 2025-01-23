pub type FourCharCode = u32;

pub type ResType = FourCharCode; // ??
pub type UniChar = u16;

pub fn four_cc_to_string(bytes: [u8; 4]) -> String {
    let mut res = String::with_capacity(4);

    for b in bytes {
        let ch = if b >= b' ' && b <= b'~' {
            unsafe { char::from_u32_unchecked(b as u32) }
        } else {
            '.'
        };
        res.push(ch);
    }

    res
}

pub fn four_cc_to_str(bytes: &mut [u8; 4]) -> &str {
    for i in 0..4 {
        let b = unsafe { bytes.get_unchecked_mut(i) };
        if b < &mut b' ' || b > &mut b'~' {
            *b = b'.';
        };
    }

    unsafe { &std::str::from_utf8_unchecked(bytes) }
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
