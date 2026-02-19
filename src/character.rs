pub(crate) enum Character {
    NormalWhitespace,
    LineFeed,
    CarriageReturn,
    SingleByte,
    MultiByte { len: u8 },
}

/// Binary extracted from [std](https://doc.rust-lang.org/src/core/str/validations.rs.html#36).
/// Added functionality to quickly discern between whitespaces.
#[allow(clippy::inline_always)]
#[inline(always)]
pub(crate) const fn get_char_metadata(first_byte: u8) -> Character {
    match first_byte {
        SPACE | TAB | FORM_FEED => Character::NormalWhitespace,
        LINE_FEED => Character::LineFeed,
        CARRIAGE_RETURN => Character::CarriageReturn,
        0b0000_0000..=0b0111_1111 => Character::SingleByte,
        0b1000_0000..=0b1101_1111 => Character::MultiByte { len: 2 },
        0b1110_0000..=0b1110_1111 => Character::MultiByte { len: 3 },
        0b1111_0000..=0b1111_1111 => Character::MultiByte { len: 4 },
    }
}

#[allow(clippy::cast_possible_truncation)]
const SPACE: u8 = ' ' as u32 as u8;
#[allow(clippy::cast_possible_truncation)]
const TAB: u8 = '\t' as u32 as u8;
#[allow(clippy::cast_possible_truncation)]
const FORM_FEED: u8 = '\x0C' as u32 as u8;

#[allow(clippy::cast_possible_truncation)]
pub(crate) const LINE_FEED: u8 = '\n' as u32 as u8;
#[allow(clippy::cast_possible_truncation)]
pub(crate) const CARRIAGE_RETURN: u8 = '\r' as u32 as u8;
