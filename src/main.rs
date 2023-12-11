use axum::{
    extract,
    extract::Path,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use reqwest;
use serde::Deserialize;
use serde_json::Value;

use std::{cmp::min, collections::HashMap};
use tower_http::services::ServeDir;



// Task -1
async fn hello_world() -> &'static str {
    "Hello, world!"
}
// Task -1 Extra
async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

// Task 1 + Extra
async fn cube_the_bits(Path(nums): Path<String>) -> String {
    let mut numbers: Vec<&str> = nums.split('/').collect();

    if numbers.last().unwrap().is_empty() {
        numbers.pop();
    }

    println!("{numbers:?}");

    let numbers = numbers
        .iter()
        .map(|n| n.parse::<i64>().unwrap())
        .fold(0, |acc, n| acc ^ n)
        .pow(3)
        .to_string();

    numbers
}
// Task 4
#[derive(Deserialize, Debug)]
struct Reindeer {
    name: String,
    strength: i32,
}
async fn combined_strength(extract::Json(payload): extract::Json<Vec<Reindeer>>) -> String {
    println!("{:?}", payload);

    let mut sum = 0;

    for deer in payload {
        sum += deer.strength;
    }

    sum.to_string()
}
// task 4 extra

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ReindeerContest {
    pub name: String,
    pub strength: i64,
    pub speed: f64,
    pub height: i64,
    pub antler_width: i64,
    pub snow_magic_power: i64,
    pub favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    pub candies: i64,
}
async fn cursed_candy_eating_contest(extract::Json(payload): extract::Json<Vec<ReindeerContest>>) {

    todo!();
    println!("{payload:?}");
    // let fastest = payload.iter().max_by_key(|r| r.speed).unwrap();
    let consumer = payload.iter().max_by_key(|r| r.candies).unwrap();
    println!("{consumer:?}");
}

// Task 7
async fn based_encoding(header: HeaderMap) -> String {
    println!("{:?}", header["cookie"]);

    let mut recipe: &str = header["cookie"].to_str().unwrap();
    recipe = &recipe[7..];

    let decoded = general_purpose::STANDARD.decode(recipe).unwrap();

    std::str::from_utf8(&decoded).unwrap().to_owned()

    // "test".to_owned()
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Kitchen {
    pub recipe: HashMap<String, i64>,
    pub pantry: HashMap<String, i64>,
}

// Task 7 extra
async fn secret_cookie_recipe(header: HeaderMap) -> String {
    println!("{:?}", header["cookie"]);

    let mut recipe: &str = header["cookie"].to_str().unwrap();
    recipe = &recipe[7..];

    let decoded = general_purpose::STANDARD.decode(recipe).unwrap();
    let json_pantry = std::str::from_utf8(&decoded).unwrap().to_owned();
    let kitchen = serde_json::from_str::<Kitchen>(&json_pantry).unwrap();

    let min_cookies = kitchen.recipe.iter().fold(i64::MAX, |a, (k, v)| {
        min(a, kitchen.pantry.get(k).unwrap_or(&0) / v)
    });
    println!("{:?}", min_cookies);

    println!("{:?}", kitchen.pantry.iter());
    "test".to_owned()
}

async fn poke_weight(Path(pokedexnum): Path<u64>) -> String {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokedexnum);
    let res = reqwest::get(url).await.unwrap();
    let body = res.text().await.unwrap();

    let parse_json: Value = serde_json::from_str(&body).unwrap();

    let weight = &parse_json["weight"].as_i64().unwrap();

    // println!("{:?}",body);
    println!("{parse_json:?}");
    println!("{weight:?}");

    (weight / &10).to_string()
}

async fn poke_momentum(Path(pokedexnum): Path<u64>) -> String {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokedexnum);
    let res = reqwest::get(url).await.unwrap();
    let body = res.text().await.unwrap();

    let parse_json: Value = serde_json::from_str(&body).unwrap();

    let weight = &parse_json["weight"].as_f64().unwrap();

    // v^2 = u^2 + 2aS

    ((weight / &10.0) * (9.825 * 20.0 as f64).sqrt()).to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/*nums", get(cube_the_bits))
        .route("/4/strength", post(combined_strength))
        .route("/4/contest", post(cursed_candy_eating_contest))
        .route("/7/decode", get(based_encoding))
        .route("/7/bake", get(secret_cookie_recipe))
        .route("/8/weight/:pokedexnum", get(poke_weight))
        .route("/8/drop/:pokedexnum", get(poke_momentum))
        .nest_service("11/assets", ServeDir::new("assets"));



    Ok(router.into())
}
