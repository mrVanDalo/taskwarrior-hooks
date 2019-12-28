use serde_json::Value;

struct Tag {
    name: String,
    keywords: Vec<String>,
}

fn main() {
    // just for testing
    let known_tags = vec![
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

    // just for testing
    let original = r#"
        {
            "name": "palo",
            "description" : "this is the description to search"
        }"#;

    parse_and_render(&original, &known_tags);
    ()
}

// parse a string and renders a string with updated tags part
fn parse_and_render(original: &str, known_tags: &Vec<Tag>) -> String {
    // Parse the string of data into serde_json::Value.
    let mut parsed_original: Value = serde_json::from_str(original).unwrap();

    // extract tags
    let tags = match parsed_original["description"] {
        Value::String(ref description) => contains_tags(&description, &known_tags),
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

    #[test]
    fn test_add_tags() {
        let known_tags = vec![
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

        assert_eq!(parse_and_render(&original, &known_tags), compact);
    }

    #[test]
    fn test_merge_tags() {
        let known_tags = vec![
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

        assert_eq!(parse_and_render(&original, &known_tags), compact);
    }
}
