use anyhow::{Context, Result};
use deadpool_postgres::GenericClient;
use thiserror::Error;

#[derive(Error, Debug, Default)]
#[error("empty artists vector specified")]
pub struct NoArtistsError;

pub async fn find_album<C: GenericClient>(
    conn: &C,
    title: &str,
    artists: &[i32],
) -> Result<Option<i32>> {
    if artists.is_empty() {
        return Err(NoArtistsError.into());
    }

    let stmt = conn
        .prepare(
            "
            SELECT a.id FROM album a
            INNER JOIN album_artist aa ON a.id = aa.album_id
            WHERE a.title = $1 AND aa.artist_id = ANY($2)
            GROUP BY a.id
            HAVING COUNT(DISTINCT aa.artist_id) = $3
            ",
        )
        .await
        .with_context(|| "failed to prepare find album query")?;

    let result = conn
        .query_opt(&stmt, &[&title, &artists, &(artists.len() as i64)])
        .await
        .with_context(|| format!("failed to execute find album query {title}"))?;

    Ok(result.map(|row| row.get("id")))
}

pub async fn insert_album<C: GenericClient>(
    conn: &mut C,
    title: &str,
    artist_ids: &[i32],
) -> Result<i32> {
    if let Some(id) = find_album(conn, title, artist_ids).await? {
        return Ok(id);
    }

    let tx = conn.transaction().await?;

    let stmt = tx
        .prepare("INSERT INTO album (title, slug) VALUES ($1, $2) RETURNING id;")
        .await
        .with_context(|| "failed to prepare insert album query")?;

    let slug = slug::slugify(title);

    let row = tx
        .query_one(&stmt, &[&title, &slug])
        .await
        .with_context(|| format!("failed to execute insert album query: {title}"))?;

    let album_id: i32 = row.get("id");

    // TODO: handle credited_as, join_phrase and artist_order
    let stmt = tx
        .prepare(
            "
        INSERT INTO album_artist (album_id, artist_id, artist_order)
        VALUES ($1, $2, $3)",
        )
        .await?;

    for artist_id in artist_ids {
        tx.execute(&stmt, &[&album_id, artist_id, &0]).await?;
    }

    tx.commit().await?;

    Ok(album_id)
}
