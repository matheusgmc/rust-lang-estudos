use dotenvy::var;
use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let database_url = var("DATABASE_URL").expect("database_url must be set");
    let connection = Database::connect(database_url)
        .await
        .expect("Failed to setup the database");

    Migrator::up(&connection, None).await?;

    Ok(connection)
}
