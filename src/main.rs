use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph{
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed = serde_json::from_str(raw_json).unwrap();
    parsed
}

fn main() {
    let json = r#"
    {
        "article": "fjewoi fwefeiow ewfniowf fewoifej ejfowijf fneoiwn",
        "author": "Fewfw fewfw",
        "paragraph": [
            {
                "name": "ffew"
            },
            {
                "name": "fewfew"
            },
            {
                "name": "fewfew"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!("name of the first article: {}", parsed.paragraph[0].name);
}