use anyhow::{Context, Result};
use deadpool_postgres::GenericClient;
use thiserror::Error;

#[derive(Error, Debug, Default)]
#[error("empty artists vector specified")]
pub struct NoArtistsError;

pub async fn find_track<C: GenericClient>(
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
            SELECT t.id FROM track t
            INNER JOIN track_artist ta ON t.id = ta.track_id
            WHERE t.title = $1
            GROUP BY t.id
            HAVING ARRAY_AGG(DISTINCT ta.artist_id) @> $2
            AND ARRAY_LENGTH(ARRAY_AGG(DISTINCT ta.artist_id), 1) = $3
            ",
        )
        .await
        .with_context(|| "failed to prepare find_track query")?;

    let result = conn
        .query_opt(&stmt, &[&title, &artists, &(artists.len() as i32)])
        .await
        .with_context(|| format!("failed to find track {title} with artists {:?}", artists))?;

    Ok(result.map(|row| row.get("id")))
}

pub struct NewTrack {
    pub title: String,
    pub artist_ids: Vec<i32>,
    pub length: Option<i32>,
    pub album_id: Option<i32>,
    pub album_track_number: Option<i32>,
}

pub async fn insert_track<C: GenericClient>(conn: &mut C, new_track: NewTrack) -> Result<i32> {
    if new_track.artist_ids.is_empty() {
        return Err(NoArtistsError.into());
    }

    // Should we check matches against album too?
    if let Some(id) = find_track(conn, &new_track.title, &new_track.artist_ids).await? {
        return Ok(id);
    }

    let tx = conn.transaction().await?;

    let stmt = tx
        .prepare("INSERT INTO track (title, slug, length) VALUES ($1, $2, $3) RETURNING id")
        .await?;

    let slug = slug::slugify(&new_track.title);

    let row = tx
        .query_one(&stmt, &[&new_track.title, &slug, &new_track.length])
        .await?;

    let track_id: i32 = row.get("id");

    // TODO: handle credited_as, join_phrase and artist_order
    let stmt = tx
        .prepare(
            "
        INSERT INTO track_artist (track_id, artist_id, artist_order)
        VALUES ($1, $2, $3)",
        )
        .await?;

    for artist_id in &new_track.artist_ids {
        tx.execute(&stmt, &[&track_id, artist_id, &0]).await?;
    }

    if let Some(album_id) = new_track.album_id {
        let stmt = tx
            .prepare(
                "INSERT INTO album_track (album_id, track_id, track_number) VALUES ($1, $2, $3)",
            )
            .await?;

        tx.execute(
            &stmt,
            &[
                &album_id,
                &track_id,
                &new_track.album_track_number.expect("track number missing"),
            ],
        )
        .await?;
    }

    tx.commit().await?;

    Ok(track_id)
}
