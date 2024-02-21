use crate::structs::{ImageDataResponse, ImageState, Metadata};
use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use std::{fmt::Debug, path::PathBuf};

pub async fn connect(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePool::connect(database_url).await?;
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<ImageDataResponse>> {
    let list = sqlx::query!(
        r#"
            SELECT id, directory_path FROM images;
        "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| {
        Ok(ImageDataResponse {
            id: row.id.unwrap() as u32,
            path: row.directory_path,
        })
    })
    .collect();

    #[cfg(feature = "log")]
    log("LIST", Some(&list));

    list
}

pub async fn insert(
    directory_path: &PathBuf,
    image_name: &str,
    store_name: &str,
    annotations_name: Option<&str>,
    metadata: Vec<Metadata>,
    pool: &SqlitePool,
) -> Result<()> {
    let directory_path = directory_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Could not convert directory path to string."))?;

    let mut transaction = pool.begin().await?;

    let result = sqlx::query!(
        r#"
            INSERT INTO images (directory_path, image_name, store_name, annotations_name)
            VALUES ($1, $2, $3, $4)
            RETURNING id;
        "#,
        directory_path,
        image_name,
        store_name,
        annotations_name
    )
    .fetch_one(&mut *transaction)
    .await?;

    #[cfg(feature = "log")]
    log::<()>(&format!("INSERT <Image: {:?}>", result.id), None);

    for m in metadata {
        sqlx::query!(
            r#"
                INSERT INTO metadata (image_id, level, cols, rows, width, height)
                VALUES ($1, $2, $3, $4, $5, $6);
            "#,
            result.id,
            m.level,
            m.cols,
            m.rows,
            m.width,
            m.height
        )
        .execute(&mut *transaction)
        .await?;

        #[cfg(feature = "log")]
        log::<()>(
            &format!("INSERT <Metadata: {}:{}>", result.id, m.level),
            None,
        );
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn contains(directory_path: &str, pool: &SqlitePool) -> bool {
    let contains = sqlx::query!(
        r#"
            SELECT * FROM images WHERE directory_path = $1;
        "#,
        directory_path,
    )
    .fetch_one(pool)
    .await
    .is_ok();

    #[cfg(feature = "log")]
    log(
        &format!("CONTAINS <Image: {}>", directory_path),
        Some(&contains),
    );

    contains
}

pub async fn get_paths(
    id: u32,
    pool: &SqlitePool,
) -> Result<(PathBuf, String, String, Option<String>)> {
    let paths = sqlx::query!(
        r#"
            SELECT directory_path, image_name, store_name, annotations_name
            FROM images
            WHERE id = $1;
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    #[cfg(feature = "log")]
    log(&format!("GET <Paths: {}>", id), Some(&paths));

    Ok((
        PathBuf::from(paths.directory_path),
        paths.image_name,
        paths.store_name,
        paths.annotations_name,
    ))
}

pub async fn get_metadata(id: u32, pool: &SqlitePool) -> Result<Vec<Metadata>> {
    // Unchecked is used here to avoid having to convert from i64 to u32.
    // This is fine because we know the values going into the database are u32
    // so as long as the database is not tampered with, this is a fine assumption.
    let metadata = sqlx::query_as_unchecked!(
        Metadata,
        r#"
            SELECT level, cols, rows, width, height
            FROM metadata
            WHERE image_id = $1
            ORDER BY level ASC;
        "#,
        id
    )
    .fetch_all(pool)
    .await?;

    #[cfg(feature = "log")]
    log(&format!("GET <Metadata: {}>", id), Some(&metadata));

    Ok(metadata)
}

pub async fn get(id: u32, pool: &SqlitePool) -> Result<ImageState> {
    let paths = get_paths(id, pool).await?;
    let metadata = get_metadata(id, pool).await?;

    let state = ImageState {
        directory_path: paths.0.into(),
        image_name: paths.1.into(),
        store_name: paths.2.into(),
        annotations_name: paths.3,
        metadata,
    };

    Ok(state)
}

pub async fn remove(id: u32, pool: &SqlitePool) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM images WHERE id = $1;
        "#,
        id
    )
    .execute(pool)
    .await?;

    #[cfg(feature = "log")]
    log::<()>(&format!("DELETE <Image: {}>", id), None);

    Ok(())
}

#[cfg(feature = "log")]
fn log<T: Debug>(operation: &str, result: Option<&T>) {
    print!("Database <{}>", operation);
    if let Some(result) = result {
        print!(": {:#?}", result);
    }
    println!("\n");
}
