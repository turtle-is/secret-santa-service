extern crate actix;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix::prelude::*;
use actix_web::{http, server, App, Error, HttpRequest, HttpResponse};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Group {
    name: String,
    members: Vec<String>,
}

struct AppState {
    groups: HashMap<String, Group>,
}

fn create_group((group, state): (Json<Group>, State<AppState>)) -> HttpResponse {
    let group_name = group.name.clone();
    state.groups.insert(group_name, group.into_inner());
    HttpResponse::Ok().into()
}

fn main() -> std::io::Result<()> {
    let state = AppState { groups: HashMap::new() };

    server::new(move || {
        App::with_state(state.clone())
            .resource("/groups", |r| r.method(http::Method::POST).with(create_group))
    })
    .bind("127.0.0.1:8080")?
    .run()
}