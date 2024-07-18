use part1::block::{generate_genesis_block, BlockHash, BlockNonce};

#[test]
fn it_works() {
    let mut genesis_block = generate_genesis_block().unwrap();
    // println!(" === Before ===");
    // println!("{}\n", genesis_block);
    // println!("{:?}", new_block.as_bincode()?);
    // println!("{:?}", hex::encode(new_block.hash()?));
    genesis_block.mining().unwrap();
    // new_block.set_nonce(BlockNonce::from(255));
    // println!(" === After ===");
    println!("Genesis block: {}", genesis_block);
    println!("Genesis block: {:?}", genesis_block);
    let result = genesis_block.verify();
    assert!(result.is_ok());
}

#[test]
fn it_fails_on_invalid_hash() {
    let mut genesis_block = generate_genesis_block().unwrap();
    genesis_block.set_current_hash(BlockHash::from(vec![]));
    let result = genesis_block.verify();

    if let Err(error) = &result {
        eprintln!("{}", error)
    }

    assert!(result.is_err());
}

#[test]
fn it_fails_on_invalid_nonce() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_nonce: u64 = rng.gen();
    let mut genesis_block = generate_genesis_block().unwrap();
    genesis_block.set_nonce(BlockNonce::from(random_nonce));
    let result = genesis_block.verify();
    if let Err(error) = &result {
        eprintln!("{}", error)
    }
    assert!(result.is_err())
}

#[test]
fn it_fails_on_new_block() {
    let genesis_block = generate_genesis_block().unwrap();
    let result = genesis_block.verify();
    if let Err(error) = &result {
        eprintln!("{}", error)
    }

    assert!(result.is_err())
}
