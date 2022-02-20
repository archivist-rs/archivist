mod lib;
use crate::lib::*;
use crate::lib::JsonFromFilename;
use rocket::response::content::Html;
use tinyjson::JsonValue;

#[macro_use] extern crate rocket;

#[get("/GetSaveType/load")]
fn hi() -> Html<String> {
    Html("templates/website_choice.html".to_html())
}

#[get("/save")]
fn spn() -> Html<String> {
    let mut base_html = "templates/save.html".to_html();
    Html(base_html)
}

#[get("/")]
fn index() -> Html<String> {
    let mut tweets = "tweets.jsonl".to_jsonl();
    let mut base_html = "templates/ui.html".to_html();

    let mut full_tweets = String::new();
    for tweet in tweets {
        println!("{tweet:?}");
        let content: String = tweet["content"].clone().try_into().unwrap();
        let id: String = match tweet["id"].clone() {
            JsonValue::String(s) => s.to_string(),
            JsonValue::Number(n) => me(JsonValue::Number(n).clone()),
            _ => panic!("Invalid JSON"),
        };
        let date: String = tweet["date"].clone().try_into().unwrap();
        let username: String = tweet["user"]["username"].clone().try_into().unwrap();
        let displayname: String = tweet["user"]["displayname"].clone().try_into().unwrap();
        let current_tweet = format!(
            r#"<div>
            <h3>{displayname} <span style='color: darkgrey'>@{username}</h3>
            <br>
            {content}
            <br><br>
            <span style="color: grey">{date}</span>
            "#);
        full_tweets = format!("{}<br>{}", full_tweets, current_tweet);
    }
    Html(format!("{base_html}, {full_tweets}"))
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, spn]).mount("/save/Save", routes![hi])
}
