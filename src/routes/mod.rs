pub mod user_routes;

use rocket::Route;

pub fn all_routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.extend(user_routes::routes());

    routes
}
