use serde::Serialize;

pub trait Hashable: Serialize {
    fn as_bincode(&self) -> anyhow::Result<Vec<u8>> {
        let encoded: Vec<u8> = bincode::serialize(&self)?;
        Ok(encoded)
    }

    fn hash(&self) -> anyhow::Result<Vec<u8>> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&self.as_bincode()?);
        let result = hasher.finalize();
        Ok(result.to_vec())
    }
}
