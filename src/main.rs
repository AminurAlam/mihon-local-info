// use reqwest::Client;
use serde_json::{json, Map, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Entry {
    pub title: String,
    pub author: String,
    pub artist: String,
    pub description: String,
    pub genre: Vec<String>,
    pub status: String,
}

// #[derive(Deserialize, Serialize)]
// pub struct Result {
//     title: Map<String, Value>,
//     status: String,
//     description: String,
//     genres: Vec<String>,
//     staff: Value
// }

const QUERY: &str = "
query ($search: String) { Page(page: 1, perPage: 5) {
    media(search: $search, type: MANGA, format_not: NOVEL) {
        title { english romaji native }
        status
        description
        genres
        staff(perPage: 25) { edges { role node { name { full } } }
    }
} } } ";

// fn print_type_of<T>(_: &T) { println!("{}", std::any::type_name::<T>()) }

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() > 0);
    let json = json!({"query": QUERY, "variables": {"search": args[1].to_string()}});

    let resp = client
        .post("https://graphql.anilist.co/")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await
        .unwrap()
        .text()
        .await;

    let results: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();
    let items: &Vec<Value> = results["data"]["Page"]["media"].as_array().unwrap();
    let mut count: isize = 1;

    for item in items {
        // println!("{}\n", serde_json::to_string_pretty(&item).unwrap());
        println!(
            "[{}] {:#} | {:#} : {}\n",
            count,
            // serde_json::to_string_pretty(&item["title"]["english"]).unwrap(),
            // serde_json::to_string_pretty(&item["title"]["native"]).unwrap(),
            item["title"]["english"].to_string(),
            item["title"]["native"].to_string(),
            item["status"].to_string()
        );
        count += 1;
    }
}
