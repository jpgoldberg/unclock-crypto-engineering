
// I really should get the blocksize from somewhere,
// but will assume 16 for now.
const BLOCKSIZE: usize = 16; // in bytes

pub(crate) fn pkcs_padder(data: &[u8]) -> Vec<u8> {

    let data_len = data.len();

    // I could do this with bitwise operations. Maybe later.
    let pbytes: usize = BLOCKSIZE - (data_len % BLOCKSIZE);
    let mut v = vec![0u8; pbytes + data_len];
    let padded = v.as_mut_slice();

    // as long as BLOCKSIZE is not greater than u8:MAX + 1 this cast is ok.
    let pad = vec![pbytes as u8; pbytes];

    padded[..data_len].copy_from_slice(data);
    padded[data_len..].copy_from_slice(&pad);

    padded.to_vec()
}

pub(crate) fn pkcs_validator(block: Vec<u8>) -> bool {
    if block.len() != BLOCKSIZE {
        return false;
    }
    let last = block[BLOCKSIZE-1];
    if last > BLOCKSIZE as u8 {
        return false;
    }
    block.ends_with(&vec![last; last as usize])
}

#[cfg(test)]
mod test {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_padder() {
        let unpadded5 = hex!("FF FF FF FF FF");
        let padded5 = hex!(
            "FF FF FF FF FF 0B 0B 0B
                                      0B 0B 0B 0B 0B 0B 0B 0B"
        );

        let result = pkcs_padder(&unpadded5);
        assert_eq!(result, padded5);

        let unpadded16 = [0u8; 16];
        let mut padded16 = unpadded16.to_vec().clone();
        padded16.append(&mut vec![16u8; 16]);

        let r16 = pkcs_padder(&unpadded16);

        assert_eq!(r16, padded16);
    }

    #[test]
    fn test_validator() {
        struct TestVector {
            name: String,
            input: Vec<u8>,
            expected: bool,
        }

        let tests = vec![
            TestVector{
                name: "Short".into(),
                input: vec![01, 02, 03, 04],
                expected: false,  

            },
            TestVector{
                name: "Long".into(),
                input: vec![01; 17],
                expected: false,  

            },
            TestVector{
                name: "four pad".into(),
                input: vec![4; 16],
                expected: true, 

            },
        ];

        for t in tests {
            let r = pkcs_validator(t.input);
            assert_eq!(r, t.expected, "test {} failed", t.name);
        }
    }

}
