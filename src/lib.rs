use std::fs;
use tinyjson::JsonValue;

pub struct JSONL {
    pub data: Vec<JsonValue>
}
impl From<String> for JSONL {
    fn from(deez: String) -> Self {
        let data = deez.split("\n");
        let mut finall = Vec::new();
        for line in data {
            let line = line.trim();
            if line == "" {
                continue;
            }
            let json_data = line.parse().expect("Bad JSON");
            finall.push(json_data);
        }
        Self { data: finall }
    }
}

pub fn read(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

pub fn me(s: JsonValue) -> String {
    let id_: f64 = s.try_into().unwrap();
    let id__: String = format!("{}", id_);
    id__
}

const hi: &'static str = r#"struct TweetData {
    content: String,
    user: JsonValue::Object,
    likes: String,
    retweets: String,
    replies: String,
    id: u64,
}"#;
