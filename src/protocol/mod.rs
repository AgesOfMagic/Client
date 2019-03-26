#![allow(warnings)]
pub mod protocol;

pub fn write_to_buff(buff: &mut [u8], word: &str) {
    for (i, char) in word.chars().enumerate() {
        buff[i] = char as u8;
    }
}
