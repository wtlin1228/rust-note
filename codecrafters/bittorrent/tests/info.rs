use bittorrent_starter_rust::{parse_torrent_file, TorrentFile, TorrentFileInfo};

#[test]
fn parse_the_torrent_file() {
    assert_eq!(
        parse_torrent_file(&[
            100, 56, 58, 97, 110, 110, 111, 117, 110, 99, 101, 53, 53, 58, 104, 116, 116, 112, 58,
            47, 47, 98, 105, 116, 116, 111, 114, 114, 101, 110, 116, 45, 116, 101, 115, 116, 45,
            116, 114, 97, 99, 107, 101, 114, 46, 99, 111, 100, 101, 99, 114, 97, 102, 116, 101,
            114, 115, 46, 105, 111, 47, 97, 110, 110, 111, 117, 110, 99, 101, 49, 48, 58, 99, 114,
            101, 97, 116, 101, 100, 32, 98, 121, 49, 51, 58, 109, 107, 116, 111, 114, 114, 101,
            110, 116, 32, 49, 46, 49, 52, 58, 105, 110, 102, 111, 100, 54, 58, 108, 101, 110, 103,
            116, 104, 105, 57, 50, 48, 54, 51, 101, 52, 58, 110, 97, 109, 101, 49, 48, 58, 115, 97,
            109, 112, 108, 101, 46, 116, 120, 116, 49, 50, 58, 112, 105, 101, 99, 101, 32, 108,
            101, 110, 103, 116, 104, 105, 51, 50, 55, 54, 56, 101, 54, 58, 112, 105, 101, 99, 101,
            115, 54, 48, 58, 232, 118, 246, 122, 42, 136, 134, 232, 243, 107, 19, 103, 38, 195, 15,
            162, 151, 3, 2, 45, 110, 34, 117, 230, 4, 160, 118, 102, 86, 115, 110, 129, 255, 16,
            181, 82, 4, 173, 141, 53, 240, 13, 147, 122, 2, 19, 223, 25, 130, 188, 141, 9, 114, 39,
            173, 158, 144, 154, 204, 23, 101, 101
        ])
        .unwrap(),
        TorrentFile {
            announce: "http://bittorrent-test-tracker.codecrafters.io/announce",
            info: TorrentFileInfo {
                name: "sample.txt",
                piece_length: 32768,
                pieces: &[
                    232, 118, 246, 122, 42, 136, 134, 232, 243, 107, 19, 103, 38, 195, 15, 162,
                    151, 3, 2, 45, 110, 34, 117, 230, 4, 160, 118, 102, 86, 115, 110, 129, 255, 16,
                    181, 82, 4, 173, 141, 53, 240, 13, 147, 122, 2, 19, 223, 25, 130, 188, 141, 9,
                    114, 39, 173, 158, 144, 154, 204, 23
                ],
                length: 92063
            }
        }
    );
}

#[test]
fn hash_the_torrent_file_info() {
    assert_eq!(
        TorrentFileInfo {
            name: "sample.txt",
            piece_length: 32768,
            pieces: &[
                232, 118, 246, 122, 42, 136, 134, 232, 243, 107, 19, 103, 38, 195, 15, 162, 151, 3,
                2, 45, 110, 34, 117, 230, 4, 160, 118, 102, 86, 115, 110, 129, 255, 16, 181, 82, 4,
                173, 141, 53, 240, 13, 147, 122, 2, 19, 223, 25, 130, 188, 141, 9, 114, 39, 173,
                158, 144, 154, 204, 23
            ],
            length: 92063
        }
        .hash()
        .unwrap(),
        "d69f91e6b2ae4c542468d1073a71d4ea13879a7f"
    )
}