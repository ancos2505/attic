use crate::block::{Block, BlockHash, BlockNonce, MINING_FACTOR};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
};

impl Block {
    pub fn mining(&mut self) -> anyhow::Result<()> {
        println!(" === MINING Block #{} ===", self.index());

        let current_hash = (*self.current_hash()).clone();
        println!("mining::current_hash -> {}", &hex::encode(&current_hash));
        println!("mining::current_hash -> {:?}", &current_hash);

        // println!("Mining...");
        let mut found_nonce: Option<u64> = None;
        // * Define multithreading params
        {
            let cpus_found: u8 = match num_cpus::get().try_into() {
                Ok(ncpus) => ncpus,
                Err(_) => 255, // max cpu
            };

            let cpus_to_mining: u8 = match cpus_found {
                0 => panic!("Impossiblestate at {}:{}", file!(), line!()),
                1 => 1,
                _ => cpus_found - 1,
            };
            let parent_arc_current_hash = Arc::new(BlockHash::from(current_hash));
            let parent_arc_is_found = Arc::new(AtomicBool::new(false));

            let number_interval: u64 = u64::MAX / u64::from(cpus_to_mining);
            let mut threads = Vec::with_capacity(cpus_to_mining.into());
            let (parent_tx, parent_rx) = mpsc::channel();
            let child_tx = parent_tx.clone();
            println!("Mining...");
            (0..cpus_to_mining).for_each(|current_cpu| {
                // let delay = (100 * u64::from(current_cpu)) + 1;
                // thread::sleep(std::time::Duration::from_millis(delay));
                let local_arc_current_hash = parent_arc_current_hash.clone();
                let local_arc_is_found = parent_arc_is_found.clone();
                let min_number_to_this_core: u64 = number_interval * u64::from(current_cpu);
                let max_number_to_this_core: u64 =
                    number_interval * (u64::from(current_cpu) + 1) + 1;
                let local_tx = child_tx.clone();

                threads.push(thread::spawn(move || {
                    // if let Err(error) = local_tx.send(42u32) {
                    //     return Err(anyhow::Error::msg(error.to_string()));
                    // }
                    let block_hash = &*local_arc_current_hash;
                    if let Some(nonce) = Block::mining_with_core(
                        local_arc_is_found.clone(),
                        current_cpu,
                        block_hash,
                        min_number_to_this_core,
                        max_number_to_this_core,
                    ) {
                        local_tx.send(nonce);
                    }
                }));
            });

            for received in parent_rx {
                println!("Got: {}", received);
                self.set_nonce(BlockNonce::from(received));
                println!("Saved nonce: {}", &*(*self.nonce()));

                // handler.join().unwrap();

                threads.into_iter().for_each(|thread| {
                    println!("Finishing thread");
                    thread
                        .join()
                        .expect("The thread creating or execution failed !");
                });
                break;
            }
        }
        // let nonce = Block::mining_single_thread(current_hash, MINING_FACTOR);

        // println!("Got: {}", nonce);

        println!("Done!");

        Ok(())
    }
    fn mining_with_core(
        local_arc_is_found: Arc<AtomicBool>,
        current_cpu: u8,
        current_hash: &BlockHash,
        min_number: u64,
        max_number: u64,
    ) -> Option<u64> {
        use byteorder::BigEndian;
        use byteorder::ByteOrder;
        use std::time::Instant;
        // use byteorder::LittleEndian;
        use sha2::{Digest, Sha256};
        // use std::io::{self, Write};
        let now = Instant::now();
        println!(
            "THREAD[{}]: MIN: {} -  MAX:{}",
            current_cpu,
            min_number,
            max_number - 1
        );
        for nonce_attemp in min_number..max_number {
            if local_arc_is_found.load(Ordering::Acquire) {
                println!("THREAD[{current_cpu}]: cancelled!");
                return None;
            }
            // print!("Nonce finding attemps:");
            if nonce_attemp % 50000 == 0 {
                println!("THREAD[{current_cpu}]: attemping {nonce_attemp}...");
            }
            let mut buf: [u8; 8] = [0; 8];

            // LittleEndian::write_u32(&mut buf, x);
            BigEndian::write_u64(&mut buf, nonce_attemp);

            let mut hasher = Sha256::new();
            hasher.update(buf);
            let h2 = hasher.finalize();

            // Compare slices
            if current_hash[0..(usize::from(MINING_FACTOR))] == h2[0..(usize::from(MINING_FACTOR))]
            {
                println!("\nElapsed: {} secs", now.elapsed().as_secs_f32());
                local_arc_is_found.store(true, Ordering::Release);
                println!("THREAD[{current_cpu}]: found {nonce_attemp}");

                return Some(nonce_attemp);
            }
        }
        println!("THREAD[{current_cpu}]: Nonce not Found");
        None
    }

    fn mining_single_thread(current_hash: &BlockHash, matching: u8) -> u32 {
        use byteorder::BigEndian;
        use byteorder::ByteOrder;
        use std::time::Instant;
        // use byteorder::LittleEndian;
        use sha2::{Digest, Sha256};
        use std::io::{self, Write};
        let now = Instant::now();
        for nonce_attemp in 0..u32::MAX {
            if nonce_attemp % 50000 == 0 {
                print!("{}", &nonce_attemp);
                print!("..");
                io::stdout().flush().expect(
                    format!("Error on trying to flush stdout: {}:{}", file!(), line!()).as_str(),
                )
            }
            let mut buf: [u8; 4] = [0; 4];

            // LittleEndian::write_u32(&mut buf, x);
            BigEndian::write_u32(&mut buf, nonce_attemp);

            let mut hasher = Sha256::new();
            hasher.update(buf);
            let h2 = hasher.finalize();

            // Compare slices
            if current_hash[0..(usize::from(matching))] == h2[0..(usize::from(matching))] {
                println!("\nElapsed: {} secs", now.elapsed().as_secs_f32());
                return nonce_attemp;
            }
        }
        0 // Not found: !Impossible state
    }
}
