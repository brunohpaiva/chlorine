use anyhow::{Context, Result};
use deadpool_postgres::GenericClient;

pub async fn insert_artist<C: GenericClient>(conn: &C, name: &str) -> Result<i32> {
    if let Some(id) = find_artist(conn, name).await? {
        return Ok(id);
    }

    let stmt = conn
        .prepare("INSERT INTO artist (name, slug) VALUES ($1, $2) RETURNING id;")
        .await
        .with_context(|| "failed to prepare insert artist query")?;

    let slug = slug::slugify(name);

    let row = conn
        .query_one(&stmt, &[&name, &slug])
        .await
        .with_context(|| format!("failed to execute insert artist query: {name}"))?;

    Ok(row.get("id"))
}

pub async fn find_artist<C: GenericClient>(conn: &C, name: &str) -> Result<Option<i32>> {
    let stmt = conn
        .prepare("SELECT id FROM artist WHERE name = $1")
        .await
        .with_context(|| "failed to prepare find artist query")?;

    let row = conn
        .query_opt(&stmt, &[&name])
        .await
        .with_context(|| format!("failed to execute find artist query: {name}"))?;

    Ok(row.map(|r| r.get("id")))
}

pub async fn get_artist_name<C: GenericClient>(conn: &C, id: i32) -> Result<Option<String>> {
    let row = conn
        .query_opt("SELECT name FROM artist WHERE id = $1", &[&id])
        .await
        .with_context(|| format!("failed to execute get artist name query: {id}"))?;

    Ok(row.map(|r| r.get("name")))
}
