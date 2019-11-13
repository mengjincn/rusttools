use chrono::prelude::*;

extern crate chrono;

use base64;

pub fn convert_timestamp_to_date(timestamp: i64) -> String {
    Local.timestamp_millis(timestamp).format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn encode_to_base64(string: &String) -> String {
    base64::encode(string)
}

pub fn decode_of_base64(string: &String) -> String {
    String::from_utf8(base64::decode(string).expect("decode base64 has error")).expect("convert vec<u8> to string has error")
}