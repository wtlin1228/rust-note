use anyhow::{Context, Ok, Result};
use serde::Serialize;
use serde_json::json;
use sha1::{Digest, Sha1};
use std::collections::HashMap;

const ENDING: u8 = b'e';
const ARRAY_START: u8 = b'l';
const INTEGER_START: u8 = b'i';
const DICTIONARY_START: u8 = b'd';
const STRING_SEPARATOR: u8 = b':';

pub fn decode_bencoded_value(encoded_value: &[u8]) -> Result<serde_json::Value> {
    let (_, decoded_value) = decode(encoded_value)?;
    let json = decoded_value.into_json()?;
    Ok(json)
}

pub struct TorrentFile<'input> {
    pub announce: &'input str,
    pub info: TorrentFileInfo<'input>,
}

#[derive(Serialize)]
pub struct TorrentFileInfo<'input> {
    pub name: &'input str,
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    #[serde(with = "serde_bytes")]
    pub pieces: &'input [u8],
    pub length: u64,
}

impl<'input> TorrentFileInfo<'input> {
    pub fn hash(&self) -> Result<String> {
        let bencoded_info_dictionary =
            serde_bencode::to_bytes(&self).context("fail to encode info dictionary")?;
        let mut hasher = Sha1::new();
        hasher.update(bencoded_info_dictionary);
        let hash = hasher.finalize();
        Ok(hex::encode(hash))
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

#[derive(Debug)]
enum Decoded<'input> {
    String(&'input [u8]),
    Integer(i64),
    Array(Vec<Decoded<'input>>),
    Dictionary(HashMap<String, Decoded<'input>>),
}

impl<'input> Decoded<'input> {
    fn into_json(&self) -> Result<serde_json::Value> {
        return Ok(match self {
            Decoded::String(bytes) => {
                json!(std::str::from_utf8(&bytes)
                    .context("fail to convert bytes into json string")?)
            }
            Decoded::Integer(n) => json!(n),
            Decoded::Array(arr) => {
                let collected: Result<Vec<serde_json::Value>> =
                    arr.into_iter().map(|item| item.into_json()).collect();
                serde_json::Value::Array(
                    collected.context("fail to collect items into json array")?,
                )
            }
            Decoded::Dictionary(dict) => {
                let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                for (key, value) in dict.iter() {
                    map.insert(
                        key.clone(),
                        value
                            .into_json()
                            .context("fail to collect values into json object")?,
                    );
                }
                serde_json::Value::Object(map)
            }
        });
    }
}

type DecodeResult<'input> = Result<(&'input [u8], Decoded<'input>)>;

fn decode(remaining: &[u8]) -> DecodeResult {
    Ok(match remaining[0] {
        ARRAY_START => decode_array(remaining)?,
        INTEGER_START => decode_integer(remaining)?,
        DICTIONARY_START => decode_dictionary(remaining)?,
        _ => decode_string(remaining)?,
    })
}

fn decode_array<'input>(remaining: &'input [u8]) -> DecodeResult {
    // array is encoded as l<inner_encoded_value>e
    //                                           |
    //                                        end_index
    let mut remaining = &remaining[1..];
    let mut items: Vec<Decoded<'input>> = vec![];
    loop {
        if remaining[0] == ENDING {
            return Ok((&remaining[1..], Decoded::Array(items)));
        }
        let (next_remaining, item) =
            decode(remaining).context("Decoding Array: fail to parse item")?;
        items.push(item);
        remaining = next_remaining;
    }
}

fn decode_integer(remaining: &[u8]) -> DecodeResult {
    // integer is encoded as i<number>e
    //                                |
    //                             end_index
    let mut end_index = 0;
    while remaining[end_index] != ENDING {
        end_index += 1;
    }
    let integer = std::str::from_utf8(&remaining[1..end_index])
        .context("Decoding Integer: size isn't in valid UTF-8 format")?
        .parse::<i64>()
        .context("Decoding Integer: fail to parse size")?;
    Ok((&remaining[end_index + 1..], Decoded::Integer(integer)))
}

fn decode_dictionary<'input>(remaining: &'input [u8]) -> DecodeResult {
    // dictionary is encoded as d<key1><value1>...<keyN><valueN>e
    //                                                          |
    //                                                       end_index
    let mut remaining = &remaining[1..];
    let mut map: HashMap<String, Decoded<'input>> = HashMap::new();
    loop {
        if remaining[0] == ENDING {
            return Ok((&remaining[1..], Decoded::Dictionary(map)));
        }
        let (next_remaining, key) =
            decode_string(remaining).context("Decoding Dictionary: fail to get key")?;
        remaining = next_remaining;
        let (next_remaining, value) =
            decode(remaining).context("Decoding Dictionary: fail to parse value")?;
        remaining = next_remaining;
        if let Decoded::String(key) = key {
            let key = std::str::from_utf8(key)
                .context("Decoding Dictionary: key isn't in valid UTF-8 format")?;
            map.insert(key.to_string(), value);
        }
    }
}

fn decode_string(remaining: &[u8]) -> DecodeResult {
    // string is encoded as <number>:<string>
    //                              |        |
    //                         colon_index   |
    //                                    end_index
    let mut colon_index = 0;
    while remaining[colon_index] != STRING_SEPARATOR {
        colon_index += 1;
    }
    let string_length = std::str::from_utf8(&remaining[..colon_index])
        .context("Decoding String: size isn't in valid UTF-8 format")?
        .parse::<i64>()
        .context("Decoding String: fail to parse size")?;
    let end_index = colon_index + 1 + string_length as usize;
    Ok((
        &remaining[end_index..],
        Decoded::String(&remaining[colon_index + 1..end_index]),
    ))
}
