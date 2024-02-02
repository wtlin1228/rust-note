use std::net::Ipv4Addr;

use bittorrent_starter_rust::{
    torrent_file::{TorrentFile, TorrentFileInfo},
    tracker::{get_request_url, parse_response, Peer, TrackerResponse},
};

#[test]
fn create_url_from_torrent_file() {
    assert_eq!(
        get_request_url(&TorrentFile {
            announce: "http://bittorrent-test-tracker.codecrafters.io/announce",
            info: TorrentFileInfo {
                name: "sample.txt",
                piece_length: 32768,
                pieces: &[
                    232, 118, 246, 122, 42, 136, 134, 232, 243, 107, 19, 103, 38, 195, 15, 162,
                    151, 3, 2, 45, 110, 34, 117, 230, 4, 160, 118, 102, 86, 115, 110, 129, 255, 16,
                    181, 82, 4, 173, 141, 53, 240, 13, 147, 122, 2, 19, 223, 25, 130, 188, 141, 9,
                    114, 39, 173, 158, 144, 154, 204, 23,
                ],
                length: 92063,
            },
        })
        .unwrap(),
        "http://bittorrent-test-tracker.codecrafters.io/announce?info_hash=%d6%9f%91%e6%b2%ae%4c%54%24%68%d1%07%3a%71%d4%ea%13%87%9a%7f&peer_id=00112233445566778899&port=6881&uploaded=0&downloaded=0&left=92063&compact=1"
    )
}

#[test]
fn create_track_response() {
    assert_eq!(
        parse_response(&[
            100, 56, 58, 99, 111, 109, 112, 108, 101, 116, 101, 105, 51, 101, 49, 48, 58, 105, 110,
            99, 111, 109, 112, 108, 101, 116, 101, 105, 49, 101, 56, 58, 105, 110, 116, 101, 114,
            118, 97, 108, 105, 54, 48, 101, 49, 50, 58, 109, 105, 110, 32, 105, 110, 116, 101, 114,
            118, 97, 108, 105, 54, 48, 101, 53, 58, 112, 101, 101, 114, 115, 49, 56, 58, 178, 62,
            82, 89, 201, 14, 165, 232, 33, 77, 201, 11, 178, 62, 85, 20, 201, 33, 101
        ])
        .unwrap(),
        TrackerResponse {
            complete: 3,
            min_interval: 60,
            incomplete: 1,
            interval: 60,
            peers: vec![
                Peer {
                    ip: Ipv4Addr::new(178, 62, 82, 89),
                    port: 51470
                },
                Peer {
                    ip: Ipv4Addr::new(165, 232, 33, 77),
                    port: 51467
                },
                Peer {
                    ip: Ipv4Addr::new(178, 62, 85, 20),
                    port: 51489
                }
            ]
        }
    )
}
