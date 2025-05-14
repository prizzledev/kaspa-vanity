use kaspa_addresses::{Address, Prefix, Version};
use secp256k1::{rand, Secp256k1, XOnlyPublicKey};
use rayon::prelude::*;
use std::io::{self, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use std::fs::File;
use serde_json::json;

fn main() {
    let mut input = String::new();

    // Ask for thread count
    print!("Enter number of threads (default 8): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let threads: usize = input.trim().parse().unwrap_or(8);
    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().unwrap();

    // Ask for match type
    input.clear();
    print!("Match type [starts|contains|ends] (default ends): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let match_type = match input.trim() {
        "starts" => "starts",
        "contains" => "contains",
        _ => "ends", // default
    };

    // Ask for pattern
    input.clear();
    print!("Enter pattern to match: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let pattern = input.trim().to_string();

    println!("🔍 Searching for Kaspa address that {} with \"{}\" using {} threads...", match_type, pattern, threads);

    let secp = Secp256k1::new();
    let counter = Arc::new(AtomicU64::new(0));
    let start = Instant::now();

    let result = (0u64..u64::MAX)
        .into_par_iter()
        .find_any(|_| {
            let count = counter.fetch_add(1, Ordering::Relaxed);
            if count % 100_000 == 0 {
                let elapsed = start.elapsed();
                println!(
                    "[{} tries in {:.2?}] Still searching...",
                    count,
                    elapsed
                );
            }

            let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
            let xonly: XOnlyPublicKey = public_key.x_only_public_key().0;
            let pub_bytes = xonly.serialize();
            let address = Address::new(Prefix::Mainnet, Version::PubKey, &pub_bytes);
            let address_str = address.to_string();

            let is_match = match match_type {
                "starts" => address_str.starts_with(&pattern),
                "contains" => address_str.contains(&pattern),
                _ => address_str.ends_with(&pattern),
            };

            if is_match {
                println!("\n🚀 Found matching address after {} tries!", count);
                println!("Kaspa Address: {}", address_str);
                println!("Private key (hex): {}", hex::encode(secret_key.secret_bytes()));

                let result_json = json!({
                    "address": address_str,
                    "secret_key": hex::encode(secret_key.secret_bytes())
                });

                let mut file = File::create("address.json").expect("Failed to create file");
                file.write_all(result_json.to_string().as_bytes()).expect("Failed to write file");

                true
            } else {
                false
            }
        });

    if result.is_none() {
        eprintln!("No address found — unlikely but possible.");
    }
}