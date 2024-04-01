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
