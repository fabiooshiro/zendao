use libsecp256k1::Error;
use sha3::{Digest, Sha3_256};


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn to_pubkey(public: &[u8]) -> Result<libsecp256k1::PublicKey, Error> {
    let mut pubkey = [4u8; 65];
    pubkey[1..65].copy_from_slice(public);
    libsecp256k1::PublicKey::parse(&pubkey)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_verify_sig() {
        let x = hex::decode("7071b4bc412e2357513698ffb42d8cc86e35ddbb2b5c5ab236a7b47213e99def16f9091e7f25d56cddaaef2cceb445d21321b40ef4f59383820a467a0c19adbf1c").unwrap();


        let pubkey_bytes = hex::decode("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap();
                                                    // 03bbba49a934014049d99a7f5c809fd0da59b1cb47dcbd0b3fe097adc5eaa5ec42
        let binding = String::from("assinatura");
        let message = binding.as_bytes();

        // ethereum signature have 65 bytes
        assert_eq!(x.len(), 65);
        println!("publickey len = {}", pubkey_bytes.len());

        let mut hasher = Sha3_256::new();
        hasher.update(message);
        let result = hasher.finalize();

        println!("hash len = {}", result.len());

        let mut data = [0u8; 64];
        data[..].copy_from_slice(&x[..64]);
        let sig = libsecp256k1::Signature::parse_standard(&data).unwrap();
        let msg = libsecp256k1::Message::parse_slice(&result).unwrap();

        // libsecp256k1::PublicKey::parse_compressed(pubkey_bytes);
        // libsecp256k1::PublicKey::parse(&pubkey_bytes);
        // let pubkey = to_pubkey(&pubkey_bytes);
        



    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
