use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use std::collections::HashMap;
use rocket_contrib::json::Json;
use rocket::State;
use std::sync::Mutex;
use crate::simulation::{Simulator};
use crate::simulation::simulation_state::SimulationState;


#[derive(Serialize, Default, Clone)]
pub struct ApiResponse {
    x: Vec<f32>,
    y: Vec<f32>,
    name: String,
}

impl ApiResponse {
    pub fn add_point(&mut self, y: f32) {
        self.x.push(self.x.len() as f32);
        self.y.push(y);
        self.name = String::from("Food Price");
    }
}

#[get("/<method>")]
fn api(method: String, simulator: State<Mutex<Simulator>>) -> Json<ApiResponse> {
    match method.as_str() {
        method => Json(simulator.lock().unwrap().update()),
    }
}

#[get("/index.html")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("", "");
    Template::render("index", context)
}

pub fn run(initial_simulation_state: SimulationState) {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index])
        .mount("/api", routes![api])
        .manage(Mutex::new(Simulator::from(initial_simulation_state)))
        .attach(Template::fairing())
        .launch();
}