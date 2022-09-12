use migration::DbErr;
mod database;
mod lib;

use lib::post;
use std::env;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let option = env::args()
        .nth(1)
        .expect("é necessario um argumento válido");

    match option.as_str() {
        "show" => post::show().await?,
        "create" => post::create().await?,
        _ => println!("opção inválida"),
    }
    Ok(())
}
