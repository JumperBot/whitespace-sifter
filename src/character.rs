pub(crate) enum Character {
    SingleByte { data: u8 },
    MultiByte { len: usize },
}

pub(crate) trait CharacterHelper {
    /// Binary extracted from [std](https://doc.rust-lang.org/src/core/str/validations.rs.html#36).
    fn get_char_metadata(self) -> Character;
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) const LINE_FEED: u8 = '\n' as u32 as u8;
#[allow(clippy::cast_possible_truncation)]
pub(crate) const CARRIAGE_RETURN: u8 = '\r' as u32 as u8;

impl CharacterHelper for u8 {
    /// Binary extracted from [std](https://doc.rust-lang.org/src/core/str/validations.rs.html#36).
    #[inline]
    fn get_char_metadata(self) -> Character {
        match self {
            0b0000_0000..=0b0111_1111 => Character::SingleByte { data: self },
            0b1000_0000..=0b1101_1111 => Character::MultiByte { len: 2 },
            0b1110_0000..=0b1110_1111 => Character::MultiByte { len: 3 },
            0b1111_0000..=0b1111_1111 => Character::MultiByte { len: 4 },
        }
    }
}
