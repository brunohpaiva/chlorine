use anyhow::{Context, Result};
use deadpool_postgres::GenericClient;
use thiserror::Error;

pub struct NewScrobble {
    pub track_title: String,
    pub track_artists: Vec<String>,
}

pub async fn insert_scrobble<C: GenericClient>(
    conn: &mut C,
    new_scrobble: &NewScrobble,
) -> Result<()> {
    let tx = conn
        .transaction()
        .await
        .with_context(|| "failed to begin transaction")?;

    let mut artists_ids: Vec<i32> = vec![];

    for artist in new_scrobble.track_artists.iter() {
        let id = upsert_artist(&tx, artist)
            .await
            .with_context(|| format!("failed to insert artist {artist}"))?;
        artists_ids.push(id);
    }

    let track_id = upsert_track(&tx, &new_scrobble.track_title, &artists_ids)
        .await
        .with_context(|| format!("failed to insert track {}", new_scrobble.track_title))?;

    let stmt = tx
        .prepare(
            "
        INSERT INTO scrobble (utc_date_time, track_id) VALUES (CURRENT_TIMESTAMP, $1)
        ",
        )
        .await
        .with_context(|| "failed to prepare scrobble insert query")?;

    tx.execute(&stmt, &[&track_id])
        .await
        .with_context(|| format!("failed to insert scrobble for track id {}", track_id))?;

    tx.commit().await?;

    Ok(())
}

async fn upsert_artist<C: GenericClient>(conn: &C, name: &str) -> Result<i32> {
    // TODO: move away from ON CONFLICT since it calls NEXTVAL() every time,
    // resulting in gaps in the ID values.
    let stmt = conn
        .prepare(
            "INSERT INTO artist (name, slug) VALUES ($1, $2)
        ON CONFLICT (slug) DO UPDATE SET name = EXCLUDED.name
        RETURNING id;",
        )
        .await
        .with_context(|| "failed to prepare artist insert query")?;

    let slug = slug::slugify(name);

    let row = conn
        .query_one(&stmt, &[&name, &slug])
        .await
        .with_context(|| "failed to insert artist {name}")?;

    Ok(row.get("id"))
}

#[derive(Error, Debug, Default)]
#[error("empty artists vector specified")]
pub struct NoArtistsError;

async fn upsert_track<C: GenericClient>(conn: &C, title: &str, artists_ids: &[i32]) -> Result<i32> {
    let Some(main_artist_id) = artists_ids.first() else {
        return Err(NoArtistsError.into());
    };

    let track_stmt = conn
        .prepare(
            "
        INSERT INTO track (title, slug, main_artist_id) VALUES ($1, $2, $3)
        ON CONFLICT (slug) DO UPDATE SET title = EXCLUDED.title
        RETURNING id;
    ",
        )
        .await
        .with_context(|| "failed prepare track insert query")?;

    let slug = slug::slugify(title);

    let track_id: i32 = conn
        .query_one(&track_stmt, &[&title, &slug, main_artist_id])
        .await
        .with_context(|| format!("failed to insert track {}", title))?
        .get("id");

    let track_artist_stmt: deadpool_postgres::tokio_postgres::Statement = conn
        .prepare(
            "
    INSERT INTO track_artist (track_id, artist_id) VALUES ($1, $2)
    ON CONFLICT DO NOTHING",
        )
        .await
        .with_context(|| "failed to prepare track_artist insert query")?;

    for artist_id in &artists_ids[1..] {
        conn.execute(&track_artist_stmt, &[&track_id, artist_id])
            .await
            .with_context(|| {
                format!(
                    "failed to insert track_artist track_id {} artist_id {}",
                    track_id, artist_id
                )
            })?;
    }

    Ok(track_id)
}
