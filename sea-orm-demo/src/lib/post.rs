use crate::database::establish_connection;
use entity::posts;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use std::io::{stdin, Read};

pub async fn show() -> Result<(), DbErr> {
    let db = establish_connection().await.expect("not connect database");

    let results: Vec<posts::Model> = posts::Entity::find()
        .filter(posts::Column::Published.eq(1))
        .all(&db)
        .await?;

    if results.len() == 0 {
        println!("Vazio!!");
        return Ok(());
    }

    for post in results {
        println!("ID - {} | {}", post.id, post.title);
        println!("{}", post.body);
    }

    Ok(())
}

pub async fn create() -> Result<(), DbErr> {
    const EOF: &str = "CTRL+D";
    let db = establish_connection().await.expect("not connect database");

    println!("Titulo do post:");
    let mut title = String::new();

    stdin().read_line(&mut title).expect("failed write title");

    let title = title.trim_end();

    println!(
        "Escreva o conteúdo do post {} (pressione {} para finalizar)",
        title, EOF
    );
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let result = posts::ActiveModel {
        title: Set(title.to_string()),
        body: Set(body),
        ..Default::default()
    };

    let result = result.insert(&db).await?;

    println!("Post created with ID: {}", result.id);
    Ok(())
}
