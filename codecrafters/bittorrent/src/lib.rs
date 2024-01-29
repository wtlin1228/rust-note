use serde_json;
use std::str;

pub fn decode_bencoded_value(encoded_value: &[u8]) -> serde_json::Value {
    match encoded_value[0] {
        b'l' => {
            return decode_array_value(encoded_value).1;
        }
        b'i' => {
            return decode_integer_value(encoded_value).1;
        }
        b'd' => {
            return decode_dictionary_value(encoded_value).1;
        }
        _ => {
            return decode_string_value(encoded_value).1;
        }
    }
}

fn decode_string(encoded_value: &[u8]) -> (&[u8], String) {
    // string is encoded as <number>:<string>
    //                              |        |
    //                         colon_index   |
    //                                    end_index
    let mut colon_index = 0;
    while encoded_value[colon_index] != b':' {
        colon_index += 1;
    }
    let string_length = str::from_utf8(&encoded_value[..colon_index])
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let end_index = colon_index + 1 + string_length as usize;
    let string = str::from_utf8(&encoded_value[colon_index + 1..end_index])
        .unwrap()
        .to_string();
    (&encoded_value[end_index..], string)
}

fn decode_string_value(encoded_value: &[u8]) -> (&[u8], serde_json::Value) {
    // string is encoded as <number>:<string>
    //                              |        |
    //                         colon_index   |
    //                                    end_index
    let (remaining, s) = decode_string(encoded_value);
    (remaining, serde_json::Value::String(s))
}

fn decode_integer_value(encoded_value: &[u8]) -> (&[u8], serde_json::Value) {
    // integer is encoded as i<number>e
    //                                |
    //                             end_index
    let mut end_index = 0;
    while encoded_value[end_index] != b'e' {
        end_index += 1;
    }
    let integer = str::from_utf8(&encoded_value[1..end_index])
        .unwrap()
        .parse::<i64>()
        .unwrap();
    (
        &encoded_value[end_index + 1..],
        serde_json::Value::Number(serde_json::Number::from(integer)),
    )
}

fn decode_array_value(encoded_value: &[u8]) -> (&[u8], serde_json::Value) {
    // array is encoded as l<inner_encoded_value>e
    //                                           |
    //                                        end_index
    let mut encoded_value = &encoded_value[1..];
    let mut items: Vec<serde_json::Value> = vec![];
    loop {
        match encoded_value[0] {
            b'e' => return (&encoded_value[1..], serde_json::Value::Array(items)),
            b'l' => {
                let res = decode_array_value(encoded_value);
                encoded_value = res.0;
                items.push(res.1);
            }
            b'i' => {
                let res = decode_integer_value(encoded_value);
                encoded_value = res.0;
                items.push(res.1);
            }
            b'd' => {
                let res = decode_dictionary_value(encoded_value);
                encoded_value = res.0;
                items.push(res.1);
            }
            _ => {
                let res = decode_string_value(encoded_value);
                encoded_value = res.0;
                items.push(res.1);
            }
        }
    }
}

fn decode_dictionary_value(encoded_value: &[u8]) -> (&[u8], serde_json::Value) {
    // array is encoded as d<key1><value1>...<keyN><valueN>e
    //                                                     |
    //                                                  end_index
    let mut encoded_value = &encoded_value[1..];
    let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    loop {
        let key: String;
        let val: serde_json::Value;
        match encoded_value[0] {
            b'e' => return (&encoded_value[1..], serde_json::Value::Object(map)),
            _ => {
                let res = decode_string(encoded_value);
                encoded_value = res.0;
                key = res.1;
            }
        }
        match encoded_value[0] {
            b'l' => {
                let res = decode_array_value(encoded_value);
                encoded_value = res.0;
                val = res.1;
            }
            b'i' => {
                let res = decode_integer_value(encoded_value);
                encoded_value = res.0;
                val = res.1;
            }
            b'd' => {
                let res = decode_dictionary_value(encoded_value);
                encoded_value = res.0;
                val = res.1;
            }
            _ => {
                let res = decode_string_value(encoded_value);
                encoded_value = res.0;
                val = res.1;
            }
        }
        map.insert(key, val);
    }
}
