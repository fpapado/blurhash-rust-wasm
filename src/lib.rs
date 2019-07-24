#![allow(dead_code)]
#![allow(unused_variables)]
pub fn decode(blur_hash: String) {}

const ENCODE_CHARACTERS: &str =
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz#$%*+,-.:;=?@[]^_{|}~";

fn decode_base83_string(string: String) -> usize {
    let mut value: usize = 0;
    for character in string.chars() {
        match ENCODE_CHARACTERS.chars().position(|c| c == character) {
            Some(digit) => value = value * 83 + digit,

            None => (),
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decodes_size_flag() {
        assert_eq!(21, decode_base83_string(String::from("L")));
        assert_eq!(0, decode_base83_string(String::from("0")));
    }
    #[test]
    fn decodes_size_0_out_of_range() {
        let res = decode_base83_string(String::from("/"));
        assert_eq!(0, res, "Did not expect to decode size for input out of range (expected 0), but got {}", res);
    }
}
