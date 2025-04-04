use serde::Deserialize;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Result;

#[derive(Deserialize, Debug, PartialEq)]
struct Tag {
    name: String,
    keywords: Vec<String>,
}

type Tags = Vec<Tag>;

fn useage() {
    println!(
        r#"
auto-tagger <path-to-tag-map>
"#
    );
}

fn main() {
    // check arguments
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        useage();
        std::process::exit(1);
    }

    let tag_map = get_tag_map(&args[1]).unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let output = parse_and_render(&input, &tag_map);
    println!("{}", output);
}

fn get_tag_map(file: &str) -> Result<Tags> {
    // read in tag_map file
    let mut file = File::open(&file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // println!("contents {}", contents);
    let result: Tags = serde_json::from_str(contents.as_ref())?;
    Ok(result)
}

// parse a string and renders a string with updated tags part
fn parse_and_render(original: &str, tag_map: &Vec<Tag>) -> String {
    // Parse the string of data into serde_json::Value.
    let mut parsed_original: Value = serde_json::from_str(original).unwrap();

    // extract tags
    let tags = match parsed_original["description"] {
        Value::String(ref description) => contains_tags(&description, &tag_map),
        _ => Vec::new(),
    };

    // merge tags
    let mut original_tags: Vec<Value> = match &parsed_original["tags"] {
        Value::Array(tags) => tags.to_vec(),
        _ => Vec::new(),
    };
    let new_tags: Vec<Value> = tags
        .iter()
        .map(|&tag| Value::String(String::from(tag)))
        .collect();
    original_tags.extend(new_tags);
    parsed_original["tags"] = Value::Array(original_tags);

    render_json(&parsed_original)
}

// Serialize it to a JSON string.
fn render_json(result: &Value) -> String {
    serde_json::to_string(&result).unwrap().to_string()
}

// extract all tags in a given description string
fn contains_tags<'a>(description: &String, tags: &'a Vec<Tag>) -> Vec<&'a String> {
    let mut result = Vec::new();

    let words = description.split(" ");
    for word in words {
        for tag in tags {
            if tag.keywords.contains(&word.to_string()) {
                result.push(&tag.name);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use serde_json::json;

    fn tags() -> Tags {
        let tag_map = vec![
            Tag {
                name: "kaufen".to_string(),
                keywords: vec!["kaufen".to_string(), "kauf".to_string(), "buy".to_string()],
            },
            Tag {
                name: "explore".to_string(),
                keywords: vec![
                    "anschauen".to_string(),
                    "find".to_string(),
                    "search".to_string(),
                ],
            },
        ];
        tag_map
    }

    #[test]
    fn test_add_tags() {
        let tag_map = tags();

        let original = r#"
        {
            "name": "palo",
            "description" : "this is the description to search"
        }"#;

        let result = json!(
        {
            "name": "palo",
            "description" : "this is the description to search",
            "tags" : ["explore"]
        });
        let compact = format!("{}", result);

        assert_eq!(parse_and_render(&original, &tag_map), compact);
    }

    #[test]
    fn test_merge_tags() {
        let tag_map = tags();

        let original = r#"
        {
            "name": "palo",
            "description" : "this is the description to search",
            "tags" : ["hallo"]
        }"#;

        let result = json!(
        {
            "name": "palo",
            "description" : "this is the description to search",
            "tags" : ["hallo", "explore"]
        });
        let compact = format!("{}", result);

        assert_eq!(parse_and_render(&original, &tag_map), compact);
    }

    #[test]
    fn test_strange_description() {
        let tag_map = tags();

        let original = r#"
        {
            "name": "palo",
            "description" : "this : is & strange description ! to search",
            "tags" : ["hallo"]
        }"#;

        let result = json!(
        {
            "name": "palo",
            "description" : "this : is & strange description ! to search",
            "tags" : ["hallo", "explore"]
        });
        let compact = format!("{}", result);

        assert_eq!(parse_and_render(&original, &tag_map), compact);
    }

    #[test]
    fn test_parse_tag_mapping() {
        let tag_map = tags();
        assert_eq!(
            get_tag_map("./test-data/auto-tagger/valid-tag-map.json").unwrap(),
            tag_map
        );

        assert!(get_tag_map("./test-data/auto-tagger/invalid-tag-map.json").is_err());
    }
}
