#![allow(non_snake_case)]
use dioxus_web::launch;

mod app;
mod components;
mod utils;

use app::App;

fn main() {
    // launch the web app
    launch(App);
}
