use axum::extract;
use serde::Deserialize;


// Task 4
#[derive(Deserialize, Debug)]
pub struct Reindeer {
    name: String,
    strength: i32,
}
pub async fn combined_strength(extract::Json(payload): extract::Json<Vec<Reindeer>>) -> String {
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
pub async fn cursed_candy_eating_contest(extract::Json(payload): extract::Json<Vec<ReindeerContest>>) {

    todo!();
    println!("{payload:?}");
    // let fastest = payload.iter().max_by_key(|r| r.speed).unwrap();
    let consumer = payload.iter().max_by_key(|r| r.candies).unwrap();
    println!("{consumer:?}");
}
