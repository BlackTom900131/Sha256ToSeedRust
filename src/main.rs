use rand::Rng;
use sha2::{Sha256, Digest};
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    // Initialize rayon thread pool for parallelism
    rayon::ThreadPoolBuilder::new().build_global().unwrap();

    // Define charset as a slice
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    // Target hash to match
    let target_hash = hex::decode("db7683d56ed28d85529167ebcd63b8708ad8ede9aa5b913ceb2e7d0801da0535").unwrap();

    let counter = AtomicUsize::new(0);

    // Generate and test seeds in parallel
    let result = (0..).par_bridge()
        .map_init(|| rand::thread_rng(), |rng, _| {
            generate_seed_bytes(rng, charset)
        })
        .try_for_each(|seed_bytes| { 
            // let count = counter.fetch_add(1, Ordering::Relaxed) + 1;
            let hash = sha256_digest(&seed_bytes);
            // if count % 10000 == 0 {
            //     // Convert hash to hex string
            //     let hash_hex = hex::encode(&hash);
            //     println!("Seed: {}", String::from_utf8_lossy(&seed_bytes));
            //     println!("SHA-256 bytes:{}", hash_hex);
            //     println!("{} seeds generated", count);
            // }
            if hash == target_hash {
                // If match found, write seed to file
                write_seed_to_file(&seed_bytes);
                // Return an error to stop further processing
                Err(())
            } else {
                Ok(())
            }
        });

    match result {
        Err(()) => println!("Matching seed found and saved!"),
        Ok(_) => println!("No matching seed found in this run."),
    }
}

// Function to generate seed bytes using provided RNG
fn generate_seed_bytes<R: Rng + ?Sized>(rng: &mut R, charset: &[u8]) -> Vec<u8> {
    let mut seed = Vec::with_capacity(64);
    for _ in 0..64 {
        let index = rng.gen_range(0..charset.len());
        seed.push(charset[index]);
    }
    seed
}

// Function to compute SHA-256 digest
fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

// Function to write seed to a file
fn write_seed_to_file(seed: &[u8]) {
    let mut file = File::create("matching_seed.txt").expect("Unable to create file");
    let seed_hex = hex::encode(seed);
    file.write_all(seed_hex.as_bytes()).expect("Unable to write data");
    println!("Seed written to matching_seed.txt");
}