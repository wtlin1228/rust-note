use anyhow::Context;
use bittorrent_starter_rust::{
    decoder::decode_bencoded_value, torrent_file::parse_torrent_file, tracker::track,
};
use bytes::{BufMut, BytesMut};
use std::io::{Read, Write};
use std::net::TcpStream;
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
            for peer in track_result.peers {
                println!("{}", peer.to_string());
            }
        }
        // Usage: your_bittorrent.sh handshake "<torrent_file_path>" "<peer_ip>:<peer_port>"
        "handshake" => {
            let file_path = &args[2];
            let peer = &args[3];
            let contents = fs::read(file_path).unwrap();
            let torrent_file = parse_torrent_file(&contents[..]).unwrap();

            let mut buf = BytesMut::with_capacity(1 + 19 + 8 + 20 + 20);
            buf.put_u8(19 as u8); // protocol length, 1 byte
            buf.put_slice(b"BitTorrent protocol"); // protocol, 19 bytes
            buf.put_bytes(0, 8); // reserved bytes, 8 bytes
            buf.put_slice(&torrent_file.info.hash_info().unwrap()); // info hash, 20 bytes
            buf.put_slice(b"00112233445566778899"); // peer id, 20 bytes

            let mut stream = TcpStream::connect(peer)
                .context("fail to connect to peer")
                .unwrap();

            stream
                .write(&buf)
                .context("fail to send handshake message")
                .unwrap();

            let mut buf = [0; 1 + 19 + 8 + 20 + 20];
            stream
                .read(&mut buf)
                .context("fail to read handshake response")
                .unwrap();

            println!("Peer ID: {}", hex::encode(&buf[buf.len() - 20..]));
        }
        _ => println!("unknown command: {}", args[1]),
    }
}
