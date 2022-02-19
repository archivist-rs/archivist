use std::fs;
use tinyjson::JsonValue;

pub trait JsonFromFilename {
    fn to_jsonl(&self) -> Vec<JsonValue>;
}
impl JsonFromFilename for str {
    fn to_jsonl(&self) -> Vec<JsonValue> {
        let filename = self;
        println!("LOADING FILE {} !", filename);
        let data_ = fs::read_to_string(filename).expect("Could not load file!");
        let data = data_.split("\n");
        let mut finall = Vec::new();
        for line in data {
            let line = line.trim();
            if line == "" {
                continue;
            }
            let json_data = line.parse().expect("Bad JSON");
            finall.push(json_data);
        }
        finall
    }
}
