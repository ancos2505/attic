use crate::result::LibResult;
use std::fmt::Display;
use std::path::Path;

#[derive(Debug)]
pub struct SriHasherReadFile;

impl SriHasherReadFile {
    pub fn read_file(self, path: impl AsRef<Path>) -> LibResult<SriHasherHashAlgo> {
        use std::fs::File;
        use std::io::Read;

        // dbg!(&path.as_ref());

        let mut file = File::open(path)?;
        let mut file_contents = vec![];
        file.read_to_end(&mut file_contents)?;

        // dbg!(&file);
        // dbg!(String::from_utf8(file_contents.clone()));

        Ok(SriHasherHashAlgo(file_contents))
    }
}

pub struct SriHasherHashAlgo(Vec<u8>);

impl SriHasherHashAlgo {
    pub fn sha_256(self) -> LibResult<SriHasherB64Encode> {
        use sha2::{Digest, Sha256};

        let mut sha256 = Sha256::new();

        sha256.update(self.0);

        let sha256_result = sha256.finalize();

        Ok(SriHasherB64Encode(sha256_result.to_vec()))
    }
    pub fn sha_512(self) -> LibResult<SriHasherB64Encode> {
        use sha2::{Digest, Sha512};

        let mut sha512 = Sha512::new();

        sha512.update(self.0);

        let sha512_result = sha512.finalize();

        Ok(SriHasherB64Encode(sha512_result.to_vec()))
    }
}

pub struct SriHasherB64Encode(Vec<u8>);

impl Display for SriHasherB64Encode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

impl SriHasherB64Encode {
    pub fn base64_encode(self) -> LibResult<String> {
        use base64::{engine::general_purpose, Engine as _};
        let b64_outcome = general_purpose::STANDARD.encode(self.0);
        Ok(b64_outcome)
    }
}
