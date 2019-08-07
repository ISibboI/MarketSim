#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rocket_contrib;
extern crate serde_json;

use rocket_contrib::templates::Template;
use std::vec::Vec;
use rocket_contrib::serve::StaticFiles;
use std::collections::HashMap;
use rocket_contrib::json::Json;
use rocket::State;
use std::borrow::BorrowMut;
use std::sync::Mutex;

#[derive(Debug, Clone, new)]
struct Buyer {
    pub max_price: i32,
    pub expected_price: i32,
}

#[derive(Debug, Clone, new)]
struct Seller {
    pub min_price: i32,
    pub expected_price: i32,
}

#[derive(Serialize)]
struct ApiResponse {
    x: Vec<f32>,
    y: Vec<f32>,
}

#[derive(new)]
struct SimulationState {
    #[new(default)]
    buyers: Vec<Buyer>,
    #[new(default)]
    sellers: Vec<Seller>,
}

#[get("/<method>")]
fn api(method: String, state: State<Mutex<SimulationState>>) -> Json<ApiResponse> {
    match method.as_str() {
        "update" => Json(update(&mut state.lock().unwrap())),
        method => Json(update(&mut state.lock().unwrap())),
    }
}

#[get("/index.html")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("", "");
    Template::render("index", context)
}

fn update(state: &mut SimulationState) -> ApiResponse {
    ApiResponse {x: vec![0.0, 1.0, 2.0, 3.0, 4.0], y: vec![1.0, 2.0, 4.0, 9.0, 16.0]}
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index])
        .mount("/api", routes![api])
        .attach(Template::fairing())
        .launch();
}
