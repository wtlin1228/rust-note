use anyhow::{Context, Ok, Result};
use serde::Serialize;
use sha1::{Digest, Sha1};

use crate::decoder::{decode, Decoded};

#[derive(PartialEq, Debug)]
pub struct TorrentFile<'input> {
    pub announce: &'input str,
    pub info: TorrentFileInfo<'input>,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct TorrentFileInfo<'input> {
    pub name: &'input str,
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    #[serde(with = "serde_bytes")]
    pub pieces: &'input [u8],
    pub length: u64,
}

impl<'input> TorrentFileInfo<'input> {
    pub fn hash_info(&self) -> Result<String> {
        let bencoded_info_dictionary =
            serde_bencode::to_bytes(&self).context("fail to encode info dictionary")?;
        let mut hasher = Sha1::new();
        hasher.update(bencoded_info_dictionary);
        let hash = hasher.finalize();
        Ok(hex::encode(hash))
    }

    pub fn hex_pieces(&self) -> Result<Vec<String>> {
        Ok(self
            .pieces
            .chunks(20)
            .map(|chunk| hex::encode(chunk))
            .collect())
    }
}

pub fn parse_torrent_file(contents: &[u8]) -> Result<TorrentFile> {
    let decoded_value = decode(contents).context("fail to decode file contents")?.1;

    let mut announce: Option<&str> = None;
    let mut length: Option<u64> = None;
    let mut name: Option<&str> = None;
    let mut piece_length: Option<u64> = None;
    let mut pieces: Option<&[u8]> = None;
    if let Decoded::Dictionary(dict) = decoded_value {
        if let Decoded::String(s) = dict.get("announce").context("should contain announce")? {
            announce =
                Some(std::str::from_utf8(s).context("announce isn't in valid UTF-8 format")?);
        };
        if let Decoded::Dictionary(info) = dict.get("info").context("should contain info")? {
            if let Decoded::Integer(n) = info.get("length").context("should contain length")? {
                length = Some(n.to_owned() as u64);
            }
            if let Decoded::String(s) = info.get("name").context("should contain name")? {
                name = Some(std::str::from_utf8(s).context("name isn't in valid UTF-8 format")?);
            }
            if let Decoded::Integer(n) = info
                .get("piece length")
                .context("should contain piece length")?
            {
                piece_length = Some(n.to_owned() as u64);
            }
            if let Decoded::String(s) = info.get("pieces").context("should contain pieces")? {
                pieces = Some(s);
            }
        }
    }

    Ok(TorrentFile {
        announce: announce.context("fail to get announce from torrent file")?,
        info: TorrentFileInfo {
            length: length.context("fail to get info.length from torrent file")?,
            name: name.context("fail to get info.name from torrent file")?,
            piece_length: piece_length.context("fail to get info.piece_length")?,
            pieces: pieces.context("fail to get info.pieces")?,
        },
    })
}
