pub(in crate) mod block_hash;
pub(in crate) mod index;
pub(in crate) mod nonce;
pub(in crate) mod payload;
pub(in crate) mod timestamp;

//
pub use block_hash::BlockHash;
pub use index::BlockIndex;
pub use nonce::BlockNonce;
pub use payload::BlockPayload;
pub use timestamp::BlockTimestamp;
//
use crate::hashable::Hashable;
use std::fmt::Debug;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

// use self::{
//     block_hash::BlockHash, index::BlockIndex, nonce::BlockNonce, payload::BlockPayload,
//     timestamp::BlockTimestamp,
// };
pub const MINING_FACTOR: u8 = 2; // How many bytes from the beginning of the hash must match
                                 // const MAX_GENESIS_BLOCK_HEIGHT: u32 = 1; // ? How many blocks will be inside the genesis block

#[derive(Debug, Serialize, Deserialize)]
pub struct PreBlock {
    index: BlockIndex,
    payload: BlockPayload,
    prev_block_hash: BlockHash,
    timestamp: BlockTimestamp,
}
impl Hashable for PreBlock {}

#[derive(Debug, Serialize)]
pub struct VerifiableBlock<'a> {
    index: &'a BlockIndex,
    payload: &'a BlockPayload,
    prev_block_hash: &'a BlockHash,
    timestamp: &'a BlockTimestamp,
}

impl Hashable for VerifiableBlock<'_> {}

// impl From<Block> for PreBlock {
//     fn from(data: Block) -> Self {
//         let Block {
//             current_hash,
//             index,
//             nonce,
//             payload,
//             prev_block_hash,
//             timestamp,
//         } = data;
//         drop(current_hash);
//         drop(nonce);
//         PreBlock {
//             index,
//             payload,
//             prev_block_hash,
//             timestamp,
//         }
//     }
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    current_hash: BlockHash,
    index: BlockIndex,
    nonce: BlockNonce,
    payload: BlockPayload,
    prev_block_hash: BlockHash,
    timestamp: BlockTimestamp,
}

impl Block {
    pub fn try_new(
        index: BlockIndex,
        payload: BlockPayload,
        prev_block_hash: BlockHash,
        timestamp: BlockTimestamp,
    ) -> anyhow::Result<Self> {
        let preblock = PreBlock {
            index,
            payload,
            prev_block_hash,
            timestamp,
        };
        let new_hash = BlockHash::from(preblock.hash()?);
        Ok(Self {
            index: preblock.index,
            // timestamp: preblock.timestamp,
            timestamp: BlockTimestamp::try_new()?,
            prev_block_hash: preblock.prev_block_hash,
            current_hash: new_hash,
            payload: preblock.payload,
            nonce: BlockNonce::default(),
        })
    }

    /// Set the block's nonce.
    pub fn set_nonce(&mut self, nonce: BlockNonce) {
        self.nonce = nonce;
    }
    /// Set the block's current hash.
    pub fn set_current_hash(&mut self, current_hash: BlockHash) {
        self.current_hash = current_hash;
    }
    /// Get a reference to the block's hash.
    #[must_use]
    pub fn current_hash<'a>(&'a self) -> &'a BlockHash {
        &self.current_hash
    }
    /// Get a reference to the block's index.
    #[must_use]
    pub fn index(&self) -> &BlockIndex {
        &self.index
    }
    /// Get a reference to the block's nonce.
    #[must_use]
    pub fn nonce(&self) -> &BlockNonce {
        &self.nonce
    }

    /// Get a reference to the block's payload.
    #[must_use]
    pub fn payload(&self) -> &BlockPayload {
        &self.payload
    }
    /// Get a reference to the block's prev block hash.
    #[must_use]
    pub fn prev_block_hash(&self) -> &BlockHash {
        &self.prev_block_hash
    }
    /// Get a reference to the block's timestamp.
    #[must_use]
    pub fn timestamp(&self) -> &BlockTimestamp {
        &self.timestamp
    }
    // TODO: Change to borrowed (block: &Block)
    pub fn verify(&self) -> anyhow::Result<()> {
        use byteorder::{BigEndian, ByteOrder};
        use sha2::Digest;
        use sha2::Sha256;
        println!("\n === VERIFYING Block #{} ===", self.index());
        let verifiable_block = VerifiableBlock {
            index: self.index(),
            payload: self.payload(),
            prev_block_hash: self.prev_block_hash(),
            timestamp: self.timestamp(),
        };
        let hash = verifiable_block.hash()?;

        println!("Checking hash: {}", hex::encode(&hash));
        if &*(*self.current_hash()) == &hash {
            let mut buf: [u8; 8] = [0; 8];

            // LittleEndian::write_u32(&mut buf, x);
            BigEndian::write_u64(&mut buf, *(*self.nonce()));

            println!("Checking nonce: {}", self.nonce());
            let mut hasher = Sha256::new();
            hasher.update(buf);
            let hash_nonce = hasher.finalize();
            if self.current_hash()[0..(usize::from(MINING_FACTOR))]
                == hash_nonce[0..(usize::from(MINING_FACTOR))]
            {
                println!("{:?} == {:?}", self.current_hash(), hash_nonce);
                println!("Block #{} is valid!", *(*self.index()));
                Ok(())
            } else {
                Err(anyhow::Error::msg("Corrupted Block: nonce is invalid"))
            }
        } else {
            Err(anyhow::Error::msg(
                "Corrupted Block: current_hash is invalid",
            ))
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block #{index}: prev_hash: {prev_hash} | current_hash: {current_hash} | timestamp: {timestamp} | payload: \"{payload}\" | nonce: {nonce}",
            index = self.index,
            prev_hash = &self.prev_block_hash,
            current_hash = &self.current_hash,
            timestamp = &self.timestamp,
            payload = &self.payload,
            nonce= &self.nonce
        )
    }
}

// TODO: Move to Chain struct
pub fn generate_genesis_block() -> anyhow::Result<Block> {
    use sha2::{Digest, Sha512};
    let mut unique_info = "".to_string();
    {
        use sysinfo::SystemExt;
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        {
            let processors = format!("Processors: {:?}", sys.processors());
            unique_info.push_str(&processors);
        }
        {
            let temperature = format!("Temperature: {:?}", sys.components());
            unique_info.push_str(&temperature);
        }
        {
            let disks = format!("Disks: {:?}", sys.disks());
            unique_info.push_str(&disks);
        }
        {
            let network_interfaces = format!("Network Interfaces: {:?}", sys.networks());
            unique_info.push_str(&network_interfaces);
        }
        {
            let system_name = format!("System name:             {:?}", sys.name());
            unique_info.push_str(&system_name);
        }
        {
            let system_kernel_info = format!("System kernel version:   {:?}", sys.kernel_version());
            unique_info.push_str(&system_kernel_info);
        }
        {
            let system_os_version = format!("System OS version:       {:?}", sys.long_os_version());
            unique_info.push_str(&system_os_version);
        }
        {
            let system_host_name = format!("System host name:        {:?}", sys.host_name());
            unique_info.push_str(&system_host_name);
        }
        {
            let system_users = format!("System host name:        {:?}", sys.users());
            unique_info.push_str(&system_users);
        }
        {
            let processes = format!("Processes: {:?}", sys.processes());
            unique_info.push_str(&processes);
        }
    }
    {
        let current_timestamp = crate::misc::now_millis()?.to_string();
        unique_info.push_str(&current_timestamp);
    }

    let mut hasher = Sha512::new();
    hasher.update(&unique_info);
    let hashed_unique_info: Vec<u8> = hasher.finalize().to_vec();

    let payload_string = hex::encode(hashed_unique_info);
    Ok(Block::try_new(
        BlockIndex::from(0),
        BlockPayload::from(payload_string),
        BlockHash::genesis(),
        BlockTimestamp::try_new()?,
    )?)
}
