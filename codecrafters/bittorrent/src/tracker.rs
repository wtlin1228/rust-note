use anyhow::{Context, Ok, Result};
use reqwest;
use std::net::Ipv4Addr;

use crate::decoder::{decode, Decoded};
use crate::torrent_file::TorrentFile;

#[derive(Debug, PartialEq)]
pub struct TrackerResponse {
    pub complete: i64,
    pub min_interval: i64,
    pub incomplete: i64,
    pub interval: i64,
    pub peer_addr_list: Vec<PeerAddr>,
}

#[derive(Debug, PartialEq)]
pub struct PeerAddr {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl PeerAddr {
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

pub fn track(torrent_file: &TorrentFile) -> Result<TrackerResponse> {
    let url = get_request_url(&torrent_file).context("get url")?;
    let response_in_bytes = &reqwest::blocking::get(url)
        .context("request the url")?
        .bytes()
        .context("read request as bytes")?[..];
    parse_response(response_in_bytes)
}

// TODO: Make it private while still being available for testing
pub fn get_request_url(torrent_file: &TorrentFile) -> Result<String> {
    let mut url = torrent_file.announce.to_owned();
    let url_encoded_info_hash: String = torrent_file
        .info
        .url_encoded_hash_info()
        .context("get url encoded hash info")?;
    url.push_str(&format!("?info_hash={}", url_encoded_info_hash));
    url.push_str("&peer_id=00112233445566778899");
    url.push_str("&port=6881");
    url.push_str("&uploaded=0");
    url.push_str("&downloaded=0");
    url.push_str(&format!("&left={}", torrent_file.info.length));
    url.push_str("&compact=1");
    Ok(url)
}

// TODO: Make it private while still being available for testing
pub fn parse_response(response: &[u8]) -> Result<TrackerResponse> {
    let decoded_value = decode(response).context("decode response")?.1;

    let mut complete: Option<i64> = None;
    let mut min_interval: Option<i64> = None;
    let mut incomplete: Option<i64> = None;
    let mut interval: Option<i64> = None;
    let mut peers: Option<Vec<PeerAddr>> = None;

    if let Decoded::Dictionary(dict) = decoded_value {
        if let Decoded::Integer(n) = dict.get("complete").context("should contain complete")? {
            complete = Some(n.to_owned());
        };
        if let Decoded::Integer(n) = dict
            .get("min interval")
            .context("should contain min_interval")?
        {
            min_interval = Some(n.to_owned());
        };
        if let Decoded::Integer(n) = dict
            .get("incomplete")
            .context("should contain incomplete")?
        {
            incomplete = Some(n.to_owned());
        };
        if let Decoded::Integer(n) = dict.get("interval").context("should contain interval")? {
            interval = Some(n.to_owned());
        };
        if let Decoded::String(info) = dict.get("peers").context("should contain peers")? {
            let mut vec: Vec<PeerAddr> = vec![];
            for chunk in info.chunks(6) {
                vec.push(PeerAddr {
                    ip: Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]),
                    port: ((chunk[4] as u16) << 8) | chunk[5] as u16,
                })
            }
            peers = Some(vec);
        }
    }

    Ok(TrackerResponse {
        complete: complete.context("get complete")?,
        min_interval: min_interval.context("get min interval")?,
        incomplete: incomplete.context("get incomplete")?,
        interval: interval.context("get interval")?,
        peer_addr_list: peers.context("get peers")?,
    })
}
