use std::env;
mod actions;
mod database;
mod models;
mod schema;

use actions::post;

fn main() {
    let action = env::args().nth(1).expect("é necessario um argumento");

    match action.as_str() {
        "show" => post::post_actions::show(),
        "create" => post::post_actions::create(),
        "publish" => post::post_actions::publish(),
        "drafts" => post::post_actions::show_posts_drafts(),
        "delete" => post::post_actions::delete(),
        _ => println!("opção inválida"),
    }
}
