pub mod home_controller;
pub mod user_controller;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        home_controller::index,
        user_controller::index,
        user_controller::show
    ]
}
