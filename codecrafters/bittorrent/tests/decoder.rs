use bittorrent_starter_rust::decoder::decode_bencoded_value;
use serde_json::json;

macro_rules! test_decode {
    ($input:expr, $output:expr) => {
        assert_eq!(
            decode_bencoded_value($input.as_bytes()).unwrap(),
            json!($output)
        )
    };
}

#[test]
fn decode_bencoded_strings() {
    test_decode!("5:apple", "apple");
    test_decode!(
        "55:http://bittorrent-test-tracker.codecrafters.io/announce",
        "http://bittorrent-test-tracker.codecrafters.io/announce"
    )
}

#[test]
fn decode_bencoded_integers() {
    test_decode!("i2131331691e", 2131331691);
    test_decode!("i4294967300e", 4294967300i64);
    test_decode!("i-52e", -52);
}

#[test]
fn decode_bencoded_lists() {
    test_decode!("le", json!([]));
    test_decode!("l5:applei169ee", json!(["apple", 169]));
    test_decode!("lli169e5:appleee", json!([[169, "apple"]]));
    test_decode!("lli4eei5ee", json!([[4], 5]));
}

#[test]
fn decode_bencoded_dictionaries() {
    test_decode!(
        "d3:foo5:apple5:helloi52ee",
        json!({"foo":"apple","hello":52})
    );
    test_decode!(
        "d10:inner_dictd4:key16:value14:key2i42e8:list_keyl5:item15:item2i3eeee",
        json!({"inner_dict":{"key1":"value1","key2":42,"list_key":["item1","item2",3]}})
    );
}
