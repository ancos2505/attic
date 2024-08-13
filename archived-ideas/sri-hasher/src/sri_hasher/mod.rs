use self::builder::SriHasherReadFile;

pub mod builder;

#[derive(Debug, Default)]
pub struct SriHasher;

impl SriHasher {
    pub fn new() -> SriHasherReadFile {
        SriHasherReadFile
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_FILE: &str = "./misc/test_data.txt";
    #[test]
    fn ok_on_read_file() {
        let path = INPUT_FILE;
        let result = SriHasher::new().read_file(path);
        assert!(result.is_ok());
    }

    #[test]
    fn ok_on_read_file_and_sha256() {
        let expected_hex = "272e48cc2c8f3ae448ae55d86aa0d6aa7f3b2b4445dc636a932a2e607796ec46";
        let path = INPUT_FILE;
        let result = SriHasher::new().read_file(path).unwrap().sha_256();
        assert!(result.is_ok());
        let hex = result.unwrap().to_string();
        assert_eq!(expected_hex, hex);
        println!("SHA-256: {}", hex);
    }

    #[test]
    fn ok_on_read_file_and_sha512() {
        let expected_hex = "7e98f238be2064971e0018ac9f1d90d7a471578cbe044c4242b16f110055e658f43c2c943716ac07e50f276c7bb7341cd034b57e01b864e5ccc8aa887f3ba843";
        let path = INPUT_FILE;
        let result = SriHasher::new().read_file(path).unwrap().sha_512();
        assert!(result.is_ok());
        let hex = result.unwrap().to_string();
        assert_eq!(expected_hex, hex);
        println!("SHA-512: {}", hex);
    }
    #[test]
    fn ok_on_read_file_and_then_sha256_and_finaly_base64() {
        // CyberChef: https://gchq.github.io/CyberChef/#recipe=SHA2('256',64,160)From_Hex('Auto')To_Base64('A-Za-z0-9%2B/%3D')&input=SnVzdCBhIHRleHQgZmlsZSB0byBydW4gdGVzdCBhZ2FpbnN0Lgo
        let expected_base64 = "Jy5IzCyPOuRIrlXYaqDWqn87K0RF3GNqkyouYHeW7EY=";
        let path = INPUT_FILE;
        let result = SriHasher::new()
            .read_file(path)
            .unwrap()
            .sha_256()
            .unwrap()
            .base64_encode();
        assert!(result.is_ok());
        let b64_str = result.unwrap();
        println!("Base64: {}", b64_str);
        assert_eq!(expected_base64, b64_str);
    }

    #[test]
    fn ok_on_read_file_and_then_sha512_and_finaly_base64() {
        // CyberChef: https://gchq.github.io/CyberChef/#recipe=SHA2('512',64,160)From_Hex('Auto')To_Base64('A-Za-z0-9%2B/%3D')&input=SnVzdCBhIHRleHQgZmlsZSB0byBydW4gdGVzdCBhZ2FpbnN0Lgo
        let expected_base64 = "fpjyOL4gZJceABisnx2Q16RxV4y+BExCQrFvEQBV5lj0PCyUNxasB+UPJ2x7tzQc0DS1fgG4ZOXMyKqIfzuoQw==";
        let path = INPUT_FILE;
        let result = SriHasher::new()
            .read_file(path)
            .unwrap()
            .sha_512()
            .unwrap()
            .base64_encode();
        assert!(result.is_ok());
        let b64_str = result.unwrap();
        println!("Base64: {}", b64_str);
        assert_eq!(expected_base64, b64_str);
    }
}
