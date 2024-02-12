use crate::structs::{ImageState, Metadata};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;
use std::fmt::Debug;
use std::path::PathBuf;

pub async fn connect() -> Result<SqlitePool> {
    // Load environment variables from .env file
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&database_url).await?;
    sqlx::migrate!().run(&pool).await?;

    // let _ = sqlx::query!("INSERT INTO images (name, image_path, store_path, annotations_path) VALUES (
    //     'hyperplastic1',
    //     '../store/hyperplastic1/hyperplastic1.tiff',
    //     '../store/hyperplastic1/hyperplastic1.zarr',
    //     '../store/hyperplastic1/hyperplastic1.sqlite'
    // );")
    //     .execute(&pool)
    //     .await?;

    // let _ = sqlx::query!("INSERT INTO metadata (name, level, cols, rows, width, height) VALUES (
    //     'hyperplastic1',
    //     0,
    //     55, 105,
    //     57152, 107952
    // );")
    //     .execute(&pool)
    //     .await?;

    Ok(pool)
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<String>> {
    let query = sqlx::query!("SELECT name FROM images;")
        .fetch_all(pool)
        .await?;

    // ! Remove unwrap.
    let result: Result<Vec<String>> = query
        .into_iter()
        .map(|row| Ok(row.name.unwrap_or_default()))
        .collect();

    log("LIST", &result).await;

    result
}

pub async fn insert(name: &str, image_state: &ImageState, pool: &SqlitePool) -> Result<()> {
    let Some(image_path) = image_state.image_path.to_str() else {
        return Err(anyhow::anyhow!("Could not convert image path to string."));
    };
    let Some(store_path) = image_state.store_path.to_str() else {
        return Err(anyhow::anyhow!("Could not convert store path to string."));
    };
    if let Some(annotations_path) = &image_state.annotations_path {
        let Some(annotations_path) = annotations_path.to_str() else {
            return Err(anyhow::anyhow!("Could not convert annotations path to string."));
        };

        let image_insert = sqlx::query!("INSERT INTO images (name, image_path, store_path, annotations_path) VALUES ($1, $2, $3, $4);",
            name,
            image_path,
            store_path,
            annotations_path
        )
            .execute(pool)
            .await?;

        log(&format!("INSERT <Image:{}>", name), &image_insert).await;
    } else {
        let image_insert = sqlx::query!("INSERT INTO images (name, image_path, store_path) VALUES ($1, $2, $3);",
            name,
            image_path,
            store_path
        )
        .execute(pool)
        .await?;

        log(&format!("INSERT <Image:{}>", name), &image_insert).await;
    }


    for metadata in &image_state.metadata {
        let metadata_insert = sqlx::query!("INSERT INTO metadata (name, level, cols, rows, width, height) VALUES ($1, $2, $3, $4, $5, $6);",
            name,
            metadata.level,
            metadata.cols,
            metadata.rows,
            metadata.width,
            metadata.height
        )
            .execute(pool)
            .await?;

        log(&format!("INSERT <Metadata:{}:{}>", name, metadata.level), &metadata_insert).await;
    }

    Ok(())
}

pub async fn contains(name: &str, pool: &SqlitePool) -> bool {
    let result = sqlx::query!("SELECT * FROM images WHERE name = $1;", name)
        .fetch_one(pool)
        .await
        .is_ok();

    log(&format!("CONTAINS <Image: {}>", name), &result).await;

    result
}

pub async fn get_paths(name: &str, pool: &SqlitePool) -> Result<(String, String, Option<String>)> {
    let image = sqlx::query!("SELECT image_path, store_path, annotations_path FROM images WHERE name = $1;", name)
        .fetch_one(pool)
        .await?;

    log(&format!("GET <Paths: {}>", name), &image).await;

    Ok((
        image.image_path,
        image.store_path,
        image.annotations_path,
    ))
}

pub async fn get_metadata(name: &str, pool: &SqlitePool) -> Result<Vec<Metadata>> {
    let metadata = sqlx::query!("SELECT level, cols, rows, width, height FROM metadata WHERE name = $1;", name)
        .fetch_all(pool)
        .await?;

    let metadata = metadata
        .into_iter()
        .map(|row| Metadata {
            level: row.level as u32,
            cols: row.cols as u32,
            rows: row.rows as u32,
            width: row.width as u32,
            height: row.height as u32,
        })
        .collect();

    log(&format!("GET <Metadata: {}>", name), &metadata).await;

    Ok(metadata)
}

pub async fn get(name: &str, pool: &SqlitePool) -> Result<Option<ImageState>> {
    let image = sqlx::query!("SELECT * FROM images WHERE name = $1;", name)
        .fetch_one(pool)
        .await?;
    
    log(&format!("GET <Image: {}>", name), &image).await;

    // let mut metadata: Vec<Metadata> = sqlx::query_as!(Metadata, "SELECT level, cols, rows, width, height FROM metadata WHERE name = $1;", name)
    //     .fetch_all(pool)
    //     .await?;

    let metadata = sqlx::query!("SELECT level, cols, rows, width, height FROM metadata WHERE name = $1;", name)
        .fetch_all(pool)
        .await?;

    let metadata = metadata
        .into_iter()
        .map(|row| Metadata {
            level: row.level as u32,
            cols: row.cols as u32,
            rows: row.rows as u32,
            width: row.width as u32,
            height: row.height as u32,
        })
        .collect();

    log(&format!("GET <Metadata: {}>", name), &metadata).await;

    let mut state = ImageState {
        image_path: image.image_path.into(),
        store_path: image.store_path.into(),
        annotations_path: None,
        metadata,
    };

    if let Some(annotations_path) = image.annotations_path {
        state.annotations_path = Some(PathBuf::from(annotations_path));
    }

    Ok(Some(state))
}

pub async fn remove(name: &str, pool: &SqlitePool) -> Result<()> {
    let image_delete = sqlx::query!("DELETE FROM images WHERE name = $1;", name)
        .execute(pool)
        .await;

    log(&format!("DELETE <Image: {}>", name), &image_delete).await;

    let metadata_delete = sqlx::query!("DELETE FROM metadata WHERE name = $1;", name)
        .execute(pool)
        .await;

    log(&format!("DELETE <Metadata: {}>", name), &metadata_delete).await;

    Ok(())
}

pub async fn log<T: Debug>(operation: &str, query: &T) {
    println!("Database <{}>: {:?}\n", operation, query);
}
