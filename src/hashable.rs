use sha2::{Sha256, Digest };

use crate::BlockHash;

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash (&self) -> BlockHash {
        let mut hasher = Sha256::new();
        hasher.update(&self.bytes());
        let hash = hasher.finalize();

        let mut result = <[u8; 32]>::default();
        result.copy_from_slice(&hash);
        
        result
    }
}

#[cfg(test)]
mod test {
    use hex_literal::hex;
    use super::*;

    struct Data(String);

    impl Hashable for Data {
        fn bytes(&self) -> Vec<u8> {
            let mut bytes = vec![];
            bytes.extend(self.0.as_bytes());
            bytes
        }
    }

    #[test]
    fn hash_data() {
        let data: Data = Data("hello world".to_owned());

        let hash = data.hash();

        assert_eq!(hash[..],  hex!("
        b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
    ")[..])
    }
}