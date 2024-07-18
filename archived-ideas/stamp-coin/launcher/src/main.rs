// use part1::block::{block_hash::BlockHash, generate_genesis_block, nonce::BlockNonce};
// use part1::block::BlockHash;
// struct Wallet(());
// struct Address(());
// struct Peer(());
// struct Balance(());
// struct Nonce(());
// struct PreviousBlockHash(BlockHash);
// struct CurrentBlockHash(BlockHash);

// * Prod

fn main() -> anyhow::Result<()> {
    let message = r#"
    # My Blockchain - Part 1

    ## Intro
    
    ### Two main data structures:
        - The blocks in the blockchain (our sole focus in this package)
        - The transactions within the blocks (next packages)
    
    ## Generic Blockchains (with PoW support)

    Blockchain =~ chronological, sequential list of *blocks*

    ### Blocks contain this information

        - **Index:** this block's location within the list of blocks
        - **Payload:** any relevant information or events that have occurred for/in the block
        - **Timestamp:** gives our blockchain a sense of time
        - **Nonce:** special number used for mining (for PoW verification)
        - **Previous block hash:** cryptographic fingerprint of previous block
        - **Hash:** cryptographic fingerprint of all of above data concatenated together

"#;
    print!("{message}");
    let mut new_block = part1::block::generate_genesis_block()?;
    // println!("{:?}", new_block.verify()?);
    if let Err(error) = new_block.verify() {
        eprintln!("ERROR: {error:?}");
    }
    new_block.mining()?;
    new_block.verify()?;
    // println!("{:?}", hex::encode(new_block.hash()?));
    // todo!("https://youtu.be/vJdT05zl6jk?t=1629");

    Ok(())
}
