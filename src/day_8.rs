use axum::extract::Path;
use serde_json::Value;


pub async fn poke_weight(Path(pokedexnum): Path<u64>) -> String {
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

pub async fn poke_momentum(Path(pokedexnum): Path<u64>) -> String {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokedexnum);
    let res = reqwest::get(url).await.unwrap();
    let body = res.text().await.unwrap();

    let parse_json: Value = serde_json::from_str(&body).unwrap();

    let weight = &parse_json["weight"].as_f64().unwrap();

    // v^2 = u^2 + 2aS

    ((weight / &10.0) * (9.825 * 20.0 as f64).sqrt()).to_string()
}
