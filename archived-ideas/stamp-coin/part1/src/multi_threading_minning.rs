use crate::block::{Block, BlockHash, BlockNonce, MINING_FACTOR};

impl Block {
    pub(in crate) fn mining(&mut self) -> anyhow::Result<()> {
        use std::thread;
        println!(" === MINING ===");
        println!("Mining block #{}", self.index());
        let matching: u8 = 3; // How many bytes from the beginning of the hash must match?

        let current_hash = self.current_hash();
        println!(
            "mining::current_hash -> {}",
            &hex::encode(&*(*current_hash))
        );
        println!("mining::current_hash -> {:?}", &*(*current_hash));

        // 255 ^ 3 = 16_581_375 // matching -> 3
        let max_number = u32::pow(255, matching.into());

        let cpus_found: u8 = match num_cpus::get().try_into() {
            Ok(ncpus) => ncpus,
            Err(_) => 255,
        };

        let cpus_to_mining: u8 = match cpus_found {
            0 => panic!("Impossiblestate at {}:{}", file!(), line!()),
            1 => 1,
            _ => cpus_found - 1,
        };
        let current_hash = self.current_hash().clone();
        // let current_hash = BlockHash::from(current_hash.clone());
        let number_interval: u32 = max_number / u32::from(cpus_to_mining);

        let parent_arc_current_hash = Arc::new(current_hash);
        let parent_arc_found_number = Arc::new(Mutex::new(0u32));
        let parent_arc_is_found = Arc::new(Mutex::new(false));

        let result_arc_found_number = parent_arc_found_number.clone();
        let mut threads = Vec::with_capacity(cpus_to_mining.into());
        let (parent_tx, parent_rx) = mpsc::channel();
        // let child_tx: mpsc::Sender<u32> = parent_tx.clone();
        let child_tx = parent_tx.clone();
        (0..cpus_to_mining).for_each(|current_cpu| {
            let min_number_to_this_core: u32 = number_interval * u32::from(current_cpu);
            let max_number_to_this_core: u32 = number_interval * (u32::from(current_cpu) + 1);
            let local_arc_current_hash = parent_arc_current_hash.clone();
            let local_arc_found_number = parent_arc_found_number.clone();
            let local_arc_is_found = parent_arc_is_found.clone();
            // let local_arc_is_found
            let local_tx = child_tx.clone();
            threads.push(thread::spawn(move || {
                // if let Err(error) = local_tx.send(42u32) {
                //     return Err(anyhow::Error::msg(error.to_string()));
                // }
                let block_hash = &*local_arc_current_hash;
                if let Some(nonce_number) = Block::mining_with_core(
                    local_tx,
                    local_arc_is_found.clone(),
                    current_cpu,
                    block_hash,
                    min_number_to_this_core,
                    max_number_to_this_core,
                    matching,
                ) {
                    if let Ok(mut mtx_number) = local_arc_found_number.lock() {
                        if let Ok(mut is_found) = local_arc_is_found.lock() {
                            *mtx_number = nonce_number;
                            *is_found = true;
                            return Ok(());
                        }
                    }
                } else {
                    return Err(anyhow::Error::msg(
                        "One thread until the end without success",
                    ));
                }
                Ok(())
            }));
        });
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(1));
            println!("Mining...");
        });
        for received in parent_rx {
            println!("Got: {}", received);
            // handler.join().unwrap();
            threads.into_iter().for_each(|thread| {
                if let Err(error) = thread
                    .join()
                    .expect("The thread creating or execution failed !")
                {
                    eprintln!("ERROR: {error}");
                }
            });
            break;
        }

        println!("Done!");
        // let found_number = match parent_arc_found_number.lock() {
        //     Ok(number) => number,
        //     Err(error) => return Err(error),
        // };
        match result_arc_found_number.lock() {
            Ok(found_number) => {
                let nonce_number = *found_number;
                println!("Found: {}", nonce_number);
                self.set_nonce(BlockNonce::from(u64::from(nonce_number)));
            }
            Err(error) => return Err(anyhow::Error::msg(error.to_string())),
        };
        Ok(())
    }
    fn mining_with_core(
        local_tx: mpsc::Sender<u32>,
        local_arc_is_found: Arc<Mutex<bool>>,
        current_cpu: u8,
        current_hash: &BlockHash,
        min_number: u32,
        max_number: u32,
        matching: u8,
    ) -> Option<u32> {
        use byteorder::BigEndian;
        use byteorder::ByteOrder;
        // use byteorder::LittleEndian;
        use sha2::{Digest, Sha256};
        use std::io::{self, Write};
        dbg!(
            "CPU: {} MIN: {} MAX:{}",
            current_cpu,
            min_number,
            max_number - 1
        );
        for nonce_attemp in min_number..max_number {
            if nonce_attemp % 500000 == 0 {
                print!("{}", &nonce_attemp);
                print!(".");
                io::stdout().flush().unwrap();
                print!("..");
            }
            let mut buf: [u8; 4] = [0; 4];

            // LittleEndian::write_u32(&mut buf, x);
            BigEndian::write_u32(&mut buf, nonce_attemp);

            let mut hasher = Sha256::new();
            hasher.update(buf);
            let h2 = hasher.finalize();

            // Compare slices
            if current_hash[0..(usize::from(matching))] == h2[0..(usize::from(matching))] {
                // println!(
                //     "\nFOUND buf  -> {}: {:?} -> {:?}",
                //     &x,
                //     &buf,
                //     &hex::encode(&h2),
                //     // &h2
                // );
                // println!("mining::hash_found -> {:?}", &h2);
                dbg!(nonce_attemp);
                if let Err(error) = local_tx.send(nonce_attemp) {
                    eprintln!("ERROR: {error}");
                }

                return Some(nonce_attemp);
            }
            if let Ok(is_found) = local_arc_is_found.lock() {
                if *is_found {
                    return None;
                }
            }
            // ! TODO:
            // ? Why is returning nonce 0
            // todo!("Why is returning nonce 0?")
            // https://doc.rust-lang.org/book/ch16-02-message-passing.html
        }
        None
    }
}
