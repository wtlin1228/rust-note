use bittorrent_starter_rust::decode_bencoded_value;
use std::env;

// Available if you need it!
// use serde_bencode

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value.as_bytes()).unwrap();
        println!("{:?}", decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
