use serde_json::{Result, Value};

fn main() {
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

    match &v["name"] {

        Value::String("palo") => println!("hillo"),
        _ => println!("unknown"),

    }
}
