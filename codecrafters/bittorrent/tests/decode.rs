use bittorrent_starter_rust::decode_bencoded_value;
use serde_json::json;

#[test]
fn decode_bencoded_strings() {
    assert_eq!(decode_bencoded_value("5:apple".as_bytes()), json!("apple"));
    assert_eq!(
        decode_bencoded_value(
            "55:http://bittorrent-test-tracker.codecrafters.io/announce".as_bytes()
        ),
        json!("http://bittorrent-test-tracker.codecrafters.io/announce")
    );
}

#[test]
fn decode_bencoded_integers() {
    assert_eq!(
        decode_bencoded_value("i2131331691e".as_bytes()),
        json!(2131331691)
    );
    assert_eq!(
        decode_bencoded_value("i4294967300e".as_bytes()),
        json!(4294967300i64)
    );
    assert_eq!(decode_bencoded_value("i-52e".as_bytes()), json!(-52));
}

#[test]
fn decode_bencoded_lists() {
    assert_eq!(decode_bencoded_value("le".as_bytes()), json!([]));
    assert_eq!(
        decode_bencoded_value("l5:applei169ee".as_bytes()),
        json!(["apple", 169])
    );
    assert_eq!(
        decode_bencoded_value("lli169e5:appleee".as_bytes()),
        json!([[169, "apple"]])
    );
    assert_eq!(
        decode_bencoded_value("lli4eei5ee".as_bytes()),
        json!([[4], 5])
    );
}

#[test]
fn decode_bencoded_dictionaries() {
    assert_eq!(
        decode_bencoded_value("d3:foo5:apple5:helloi52ee".as_bytes()),
        json!({"foo":"apple","hello":52})
    );
    assert_eq!(
        decode_bencoded_value(
            "d10:inner_dictd4:key16:value14:key2i42e8:list_keyl5:item15:item2i3eeee".as_bytes()
        ),
        json!({"inner_dict":{"key1":"value1","key2":42,"list_key":["item1","item2",3]}})
    );
}
