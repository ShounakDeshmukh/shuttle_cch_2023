mod day_minus_1;
mod day_1;
mod day_4;
mod day_7;
mod day_8;
mod day_11;
use axum::{

    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(day_minus_1::hello_world))
        .route("/-1/error", get(day_minus_1::fake_error))
        .route("/1/*nums", get(day_1::cube_the_bits))
        .route("/4/strength", post(day_4::combined_strength))
        .route("/4/contest", post(day_4::cursed_candy_eating_contest))
        .route("/7/decode", get(day_7::based_encoding))
        .route("/7/bake", get(day_7::secret_cookie_recipe))
        .route("/8/weight/:pokedexnum", get(day_8::poke_weight))
        .route("/8/drop/:pokedexnum", get(day_8::poke_momentum))
        .nest_service("/11/assets", ServeDir::new("assets"))
        .route("/11/red_pixels", post(day_11::bull_mode));  //Task 11 



    Ok(router.into())
}
