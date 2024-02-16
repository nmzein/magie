use crate::structs::{ImageState, Metadata};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use std::{env, fmt::Debug, path::PathBuf};

pub async fn connect() -> Result<SqlitePool> {
    // Load environment variables from .env file.
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<String>> {
    let query = sqlx::query!(
        r#"
            SELECT name FROM images;
        "#
    )
        .fetch_all(pool)
        .await?;

    let result: Result<Vec<String>> = query
        .into_iter()
        .map(|row| Ok(row.name.unwrap_or_default()))
        .collect();

    #[cfg(feature = "log")]
    log("LIST", &result).await;

    result
}

pub async fn insert(name: &str, image_state: &ImageState, pool: &SqlitePool) -> Result<()> {
    let image_path = image_state.image_path.to_str()
        .ok_or_else(|| anyhow::anyhow!("Could not convert image path to string."))?;

    let store_path = image_state.store_path.to_str()
        .ok_or_else(|| anyhow::anyhow!("Could not convert store path to string."))?;

    let annotations_path = image_state.annotations_path
        .as_ref()
        .map(|path| path.to_str().ok_or_else(|| anyhow::anyhow!("Could not convert annotations path to string.")))
        .transpose()?;

    let image_insert = sqlx::query!(
        r#"
            INSERT INTO images (name, image_path, store_path, annotations_path)
            VALUES ($1, $2, $3, $4);
        "#,
        name,
        image_path,
        store_path,
        annotations_path
    )
        .execute(pool)
        .await?;

    #[cfg(feature = "log")]
    log(&format!("INSERT <Image:{}>", name), &image_insert).await;

    for metadata in &image_state.metadata {
        let metadata_insert = sqlx::query!(
            r#"
                INSERT INTO metadata (name, level, cols, rows, width, height)
                VALUES ($1, $2, $3, $4, $5, $6);
            "#,
            name,
            metadata.level,
            metadata.cols,
            metadata.rows,
            metadata.width,
            metadata.height
        )
            .execute(pool)
            .await?;

        #[cfg(feature = "log")]
        log(&format!("INSERT <Metadata:{}:{}>", name, metadata.level), &metadata_insert).await;
    }

    Ok(())
}

pub async fn contains(name: &str, pool: &SqlitePool) -> bool {
    let result = sqlx::query!(
        r#"
            SELECT * FROM images WHERE name = $1;
        "#,
        name
    )
        .fetch_one(pool)
        .await
        .is_ok();

    #[cfg(feature = "log")]
    log(&format!("CONTAINS <Image: {}>", name), &result).await;

    result
}

pub async fn get_paths(name: &str, pool: &SqlitePool) -> Result<(String, String, Option<String>)> {
    let paths = sqlx::query!(
        r#"
            SELECT image_path, store_path, annotations_path
            FROM images
            WHERE name = $1;
        "#,
        name
    )
        .fetch_one(pool)
        .await?;

    #[cfg(feature = "log")]
    log(&format!("GET <Paths: {}>", name), &paths).await;

    Ok((
        paths.image_path,
        paths.store_path,
        paths.annotations_path,
    ))
}

pub async fn get_metadata(name: &str, pool: &SqlitePool) -> Result<Vec<Metadata>> {
    // Unchecked is used here to avoid having to convert from i64 to u32.
    // This is fine because we know the values going into the database are u32
    // so as long as the database is not tampered with, this is a fine assumption.
    let metadata = sqlx::query_as_unchecked!(
        Metadata,
        r#"
            SELECT level, cols, rows, width, height
            FROM metadata
            WHERE name = $1
            ORDER BY level ASC;
        "#,
        name
    )
        .fetch_all(pool)
        .await?;

    log(&format!("GET <Metadata: {}>", name), &metadata).await;

    Ok(metadata)
}

pub async fn get(name: &str, pool: &SqlitePool) -> Result<Option<ImageState>> {
    let paths = get_paths(name, pool).await?;
    let metadata = get_metadata(name, pool).await?;

    let state = ImageState {
        image_path: paths.0.into(),
        store_path: paths.1.into(),
        annotations_path: paths.2.map(PathBuf::from),
        metadata,
    };

    Ok(Some(state))
}

pub async fn remove(name: &str, pool: &SqlitePool) -> Result<()> {
    let image_delete = sqlx::query!(
        r#"
            DELETE FROM images WHERE name = $1;
        "#,
        name
    )
        .execute(pool)
        .await;

    #[cfg(feature = "log")]
    log(&format!("DELETE <Image: {}>", name), &image_delete).await;

    Ok(())
}

pub async fn log<T: Debug>(operation: &str, query: &T) {
    println!("Database <{}>: {:?}\n", operation, query);
}