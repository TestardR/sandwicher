use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

pub async fn connect_to_db(db_url: String, migration_path: String) -> Pool<Sqlite> {
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        match Sqlite::create_database(&db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    }

    let db = SqlitePool::connect(&db_url).await.unwrap();

    let migrations = std::path::Path::new(&migration_path);
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    db
}