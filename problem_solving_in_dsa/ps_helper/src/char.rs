use std::fmt::{Binary, Debug};

/// ASCII chars use 7 bits \
/// Complex char use 32 bits \
/// Note: Pads 0 at the front if bits of c are less than 8 \
/// Note: Only useful if char bits len() <= 8 
pub fn char2bits_u8_basic(c: char) -> [u8; 8] {
    let mut bits = [0; 8];
    for i in 0..8 {
        bits[i] = ((c as u8) >> (7 - i)) & 1;
    }
    bits
}

// Useful if bits used by char <= 8  \
/// Outputs string that contains all bits in char's representation \
/// Note: char occupies 32 bits in memory 
/// and has 0s are padded to front if char requires less bit for representation 
pub fn char2bits_u8(c:char) -> String{
    format!("{:b}", c as u8)
}

/// Useful for chars has bits less than or equal to bits in T  \
/// where T can only be integer type: u32, u64 and u128. \
/// Outputs string that contains all bits in char's representation
pub fn char2bits<T>(c: char) -> String
where
    T: Binary + From<char> + Default + Clone + Debug,
{
    format!("{:b}", <char as std::convert::Into<T>>::into(c))
}

#[test]
fn test_v2() {
    dbg!(char2bits_u8_basic('C'));
    dbg!(char2bits_u8('C'));

    dbg!(char2bits::<u128>('📦'));
    dbg!(char2bits::<u32>('📦'));
    dbg!(char2bits::<u64>('📦'));
}
