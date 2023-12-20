use std::{collections::HashMap, cmp::min};

use axum::http::HeaderMap;
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;

// Task 7
pub async fn based_encoding(header: HeaderMap) -> String {
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
pub async fn secret_cookie_recipe(header: HeaderMap) -> String {
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
