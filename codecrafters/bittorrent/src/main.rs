use bittorrent_starter_rust::{decode_bencoded_value, parse_torrent_file};
use std::{env, fs};

// Available if you need it!
// use serde_bencode

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value.as_bytes()).unwrap();
        println!("{}", decoded_value.to_string());
    } else if command == "info" {
        let file_path = &args[2];
        let contents = fs::read(file_path).unwrap();
        let torrent_file = parse_torrent_file(&contents[..]).unwrap();
        println!("Tracker URL: {}", torrent_file.announce);
        println!("Length: {}", torrent_file.info.length);
        println!("Info Hash: {}", torrent_file.info.hash_info().unwrap());
        println!("Piece Length: {}", torrent_file.info.piece_length);
        println!("Piece Hashes");
        for s in torrent_file.info.hex_pieces().unwrap() {
            println!("{}", s);
        }
    } else {
        println!("unknown command: {}", args[1])
    }
}
