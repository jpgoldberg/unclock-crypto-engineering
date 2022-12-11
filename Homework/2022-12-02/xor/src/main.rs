use anyhow::{ensure, Error};
use bytes::Bytes;
use thiserror::Error;

#[derive(Error, Debug)]
enum XorError {
    #[error("Mismatched lengths")]
    MismatchedLengths,
}

/// This is only so that I can implement my own try_from and fmt
struct MyBytes(Bytes);

impl TryFrom<&str> for MyBytes {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, anyhow::Error> {
        let allowed_punc: [char; 5] = ['\n', ' ', '\t', ':', '_'];
        let stripped = s
            .chars()
            .filter(|c| !allowed_punc.contains(c))
            .collect::<String>();

        let b = hex::decode(stripped)?;

        Ok(Self(Bytes::from(b)))
    }
}

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn xor_bytes(a: Bytes, b: Bytes) -> Result<Bytes, Error> {
    ensure!(a.len() == b.len(), XorError::MismatchedLengths);
    let result = std::iter::zip(a, b)
        .map(|(a_byte, b_byte)| a_byte ^ b_byte)
        .collect::<Bytes>();

    Ok(result)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_pair() {
        let dawn_text = "Attack at dawn!";
        let dusk_text = "Attack at dusk!";

        let dawn_hex = hex::encode(dawn_text);
        let dusk_hex = hex::encode(dusk_text);

        let dawn_bytes = crate::MyBytes::try_from(dawn_hex.as_str()).unwrap();

        let dusk_bytes = crate::MyBytes::try_from(dusk_hex.as_str()).unwrap();

        let combined = crate::xor_bytes(dawn_bytes.0, dusk_bytes.0).unwrap();

        assert_eq!(hex::encode(combined), "000000000000000000000014040500");
    }
}
