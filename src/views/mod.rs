use rocket::fairing::Fairing;
use rocket_dyn_templates::Template;

pub fn init_template() -> impl Fairing {
    Template::fairing()
}
