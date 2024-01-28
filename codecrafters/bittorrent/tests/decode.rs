use bittorrent_starter_rust::decode_bencoded_value;
use serde_json::json;

#[test]
fn decode_bencoded_strings() {
    assert_eq!(decode_bencoded_value("5:apple"), json!("apple"));
    assert_eq!(
        decode_bencoded_value("55:http://bittorrent-test-tracker.codecrafters.io/announce"),
        json!("http://bittorrent-test-tracker.codecrafters.io/announce")
    );
}

#[test]
fn decode_bencoded_integers() {
    assert_eq!(decode_bencoded_value("i2131331691e"), json!(2131331691));
    assert_eq!(decode_bencoded_value("i4294967300e"), json!(4294967300i64));
    assert_eq!(decode_bencoded_value("i-52e"), json!(-52));
}

#[test]
fn decode_bencoded_lists() {
    assert_eq!(decode_bencoded_value("le"), json!([]));
    assert_eq!(
        decode_bencoded_value("l5:applei169ee"),
        json!(["apple", 169])
    );
    assert_eq!(
        decode_bencoded_value("lli169e5:appleee"),
        json!([[169, "apple"]])
    );
    assert_eq!(decode_bencoded_value("lli4eei5ee"), json!([[4], 5]));
}
