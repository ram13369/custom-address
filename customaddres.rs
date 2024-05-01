use ethkey::{generate, KeyType};
use std::time::Instant;

fn main() {
    // Prefix and suffix
    let prefix = "ram";
    let suffix = "ram";

    // Length of prefix and suffix
    let prefix_length = prefix.len() + 2;
    let suffix_length = suffix.len();

    // Variables for temporary storage
    let mut has_found_it = false;
    let mut pre_temp;
    let mut suf_temp;
    let mut a;

    let start = Instant::now();

    while !has_found_it {
        // Generate a new Ethereum key pair
        let key_pair = generate(KeyType::Secp256k1).unwrap();

        // Get the public address
        let public_address = key_pair.public_address();

        // Get the address string
        a = public_address.to_string();

        // Extract prefix and suffix from the address string
        pre_temp = a.chars().skip(2).take(prefix_length).collect();
        suf_temp = a.chars().skip(a.len() - suffix_length).take(suffix_length).collect();

        // Check if prefix and suffix match
        if pre_temp.contains(prefix) && suf_temp.contains(suffix) {
            println!("Got it!");
            println!("Private Key: {}", key_pair.private_key());
            println!("Public Key: {}", public_address);
            has_found_it = true;
        }
    }

    let end = Instant::now();

    let time = end.duration_since(start).as_secs_f64();
    println!("Execution Time: {:.2} seconds", time);
}
