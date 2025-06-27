#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod views;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(views::init_template())
        .mount("/", controllers::routes())
}
