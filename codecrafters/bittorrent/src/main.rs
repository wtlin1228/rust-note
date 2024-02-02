use bittorrent_starter_rust::{
    decoder::decode_bencoded_value, torrent_file::parse_torrent_file, tracker::track,
};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1][..];

    match command {
        // Usage: your_bittorrent.sh decode "<encoded_value>"
        "decode" => {
            let encoded_value = &args[2];
            let decoded_value = decode_bencoded_value(encoded_value.as_bytes()).unwrap();
            println!("{}", decoded_value.to_string());
        }
        // Usage: your_bittorrent.sh info "<torrent_file_path>"
        "info" => {
            let file_path = &args[2];
            let contents = fs::read(file_path).unwrap();
            let torrent_file = parse_torrent_file(&contents[..]).unwrap();
            println!("Tracker URL: {}", torrent_file.announce);
            println!("Length: {}", torrent_file.info.length);
            println!("Info Hash: {}", torrent_file.info.hex_info().unwrap());
            println!("Piece Length: {}", torrent_file.info.piece_length);
            println!("Piece Hashes");
            for s in torrent_file.info.hex_pieces().unwrap() {
                println!("{}", s);
            }
        }
        // Usage: your_bittorrent.sh peers "<torrent_file_path>"
        "peers" => {
            let file_path = &args[2];
            let contents = fs::read(file_path).unwrap();
            let torrent_file = parse_torrent_file(&contents[..]).unwrap();
            let track_result = track(torrent_file).unwrap();
            println!("{:?}", track_result);
            for peer in track_result.peers {
                println!("{}", peer.to_string());
            }
        }
        _ => println!("unknown command: {}", args[1]),
    }
}
