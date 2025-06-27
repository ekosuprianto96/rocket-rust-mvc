use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();

    Template::render("home/index", &context)
}
