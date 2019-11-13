use chrono::prelude::*;
extern crate reqwest;
extern crate chrono;


use base64;

use serde_json::{Result, Value};
#[derive(Serialize, Deserialize)]
pub struct Person{
    name: String,
    age: u8,
    is_male: bool
}

pub fn convert_timestamp_to_date(timestamp: i64) -> String {
    Local.timestamp_millis(timestamp).format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn encode_to_base64(string: &String) -> String {
    base64::encode(string)
}

pub fn decode_of_base64(string: &String) -> String {
    String::from_utf8(base64::decode(string).expect("decode base64 has error")).expect("convert vec<u8> to string has error")
}

pub fn http_test(){
    let response = reqwest::get("http://baidu.com").expect("could not make request").text().expect("cloud not read text from response");
    println!("response {}", response);
}

pub fn json_test(){
// Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data).unwrap();

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    let json_str = r#"
    {
    "name":"mengjin", "age":30, "is_male":true
    }
    "#;
    let res = serde_json::from_str(json_str);
    if res.is_ok() {
        let p: Person = res.unwrap();
        println!("you name is {}", p.name);
        println!("you age is {}", p.age);
        println!("you male is {}", p.is_male);
    } else {
        println!("Could not parse JSON");
    }

    let person:Person = Person{
        name: "mengjin".to_string(),
        age: 8,
        is_male: true
    };
    let j = serde_json::to_string(&person);
    println!("{}",j.unwrap());
}