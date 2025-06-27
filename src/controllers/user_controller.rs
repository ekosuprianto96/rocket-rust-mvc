use crate::models::user::User;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/users")]
pub fn index() -> Template {
    let mut context = HashMap::new();
    let users = vec![
        User {
            id: 1,
            name: String::from("Alice"),
        },
        User {
            id: 2,
            name: String::from("Bob"),
        },
    ];
    context.insert("users", users);
    Template::render("users/index", &context)
}

#[get("/users/<id>")]
pub fn show(id: usize) -> Template {
    let mut context: HashMap<&'static str, User> = HashMap::new();
    let user = User {
        id,
        name: format!("User {}", id),
    };
    context.insert("user", user);
    Template::render("users/show", &context)
}
