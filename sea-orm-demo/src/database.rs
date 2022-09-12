use dotenvy::dotenv;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};
use std::env;

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("database_url must be set");
    let connection = Database::connect(database_url)
        .await
        .expect("Failed to setup the database");

    Migrator::up(&connection, None).await?;

    Ok(connection)
}
