use chrono::prelude::*;

extern crate chrono;

pub fn convert_timestamp_to_date(timestamp:i64)->String {
    Local.timestamp_millis(timestamp).format("%Y-%m-%d %H:%M:%S").to_string()
}