mod actions;
mod database;
mod models;
mod schema;

use actions::{menu, post};

fn main() {
    loop {
        let option = menu::menu();
        match option {
            1 => post::post_actions::show(),
            2 => post::post_actions::create(),
            3 => post::post_actions::publish(),
            4 => post::post_actions::show_posts_drafts(),
            5 => println!("Opção 4"),
            6 => break,
            _ => println!("opção inválida"),
        }
    }
}
