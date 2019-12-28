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

    parse_and_render(&original, &known_tags)
}

fn parse_and_render(original: &str, known_tags: &Vec<Tag>) {
    // Parse the string of data into serde_json::Value.
    let mut parsed_original: Value = serde_json::from_str(original).unwrap();

    // extract tags
    let tags = match parsed_original["description"] {
        Value::String(ref description) => contains_tags(&description, &known_tags),
        _ => Vec::new(),
    };

    // merge tags
    parsed_original["tags"] = Value::Array(
        tags.iter()
            .map(|&tag| Value::String(String::from(tag)))
            .collect(),
    );

    render_json(&parsed_original);
}

// Serialize it to a JSON string.
fn render_json(result: &Value) {
    let json = serde_json::to_string(&result).unwrap();
    println!("{}", json);
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
