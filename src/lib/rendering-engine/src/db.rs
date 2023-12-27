use crate::structs::{ImageState, ImageMetadata};
use sqlx::sqlite::SqlitePool;
use anyhow::Result;
use dotenv::dotenv;
use std::env;

pub async fn connect() -> Result<SqlitePool> {
    // Load environment variables from .env file
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<String>> {
    let query = sqlx::query!("SELECT name FROM images;")
        .fetch_all(pool)
        .await?;

    println!("LIST Query: {:?}", query);

    let result: Result<Vec<String>> = query
        .into_iter()
        .map(|row| Ok(row.name.unwrap_or_default()))
        .collect();

    result
}

pub async fn insert(name: &str, image_state: &ImageState, pool: &SqlitePool) -> Result<()> {
    let image_path = image_state.image_path.to_str().unwrap();
    let store_path = image_state.store_path.to_str().unwrap();
    
    match sqlx::query!("INSERT INTO images (name, image_path, store_path, cols, rows) VALUES ($1, $2, $3, $4, $5);",
        name,
        image_path,
        store_path,
        image_state.metadata.cols,
        image_state.metadata.rows)
        .execute(pool)
        .await {
        Ok(_) => println!("Inserted!"),
        Err(e) => println!("Error: {}", e),
    };

    Ok(())
}

pub async fn contains(id: &str, pool: &SqlitePool) -> bool {
    get(id, pool).await.is_ok()
}

pub async fn get(id: &str, pool: &SqlitePool) -> Result<Option<ImageState>> {
    let query = sqlx::query!("SELECT * FROM images WHERE name = $1;", id)
        .fetch_one(pool)
        .await?;

    println!("GET Query: {:?}", query);

    Ok(Some(ImageState {
        image_path: query.image_path.into(),
        store_path: query.store_path.into(),
        metadata: ImageMetadata {
            cols: query.cols.try_into().unwrap(),
            rows: query.rows.try_into().unwrap(),
        },
    }))
}

pub async fn remove(id: &str, pool: &SqlitePool) -> bool {
    let query = sqlx::query!("DELETE FROM images WHERE name = $1;", id)
        .execute(pool)
        .await;

    // println!("REMOVE Query: {:?}", query);

    return query.is_ok();
}
