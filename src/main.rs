mod lib;
use crate::lib::*;
use crate::lib::JsonFromFilename;

fn main() {
    println!("{:?}", "tweets.jsonl".load_jsonl());
}
