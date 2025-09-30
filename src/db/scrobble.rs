use anyhow::{Context, Result};
use deadpool_postgres::GenericClient;

use crate::db::{
    album::insert_album,
    artist::insert_artist,
    track::{NewTrack, insert_track},
};

pub struct NewScrobble {
    pub track_title: String,
    pub track_artists: Vec<String>,
    pub album_title: Option<String>,
    pub album_artists: Option<Vec<String>>,
}

pub async fn insert_scrobble<C: GenericClient>(
    conn: &mut C,
    new_scrobble: &NewScrobble,
) -> Result<()> {
    let mut tx = conn
        .transaction()
        .await
        .with_context(|| "failed to begin transaction")?;

    let mut track_artist_ids: Vec<i32> = vec![];
    for artist in &new_scrobble.track_artists {
        let id = insert_artist(&tx, artist)
            .await
            .with_context(|| format!("failed to insert artist {artist}"))?;
        track_artist_ids.push(id);
    }

    let album_id: Option<i32> = match (&new_scrobble.album_title, &new_scrobble.album_artists) {
        (Some(album_title), Some(album_artists)) => {
            // TODO: this could reuse the artist ids from the track if they are the same
            let mut album_artist_ids: Vec<i32> = vec![];
            for artist in album_artists {
                let id = insert_artist(&tx, artist)
                    .await
                    .with_context(|| format!("failed to insert artist {artist}"))?;
                album_artist_ids.push(id);
            }

            Some(
                insert_album(&mut tx, album_title, &album_artist_ids)
                    .await
                    .with_context(|| format!("failed to insert album {album_title}"))?,
            )
        }
        _ => None,
    };

    let track_id = insert_track(
        &mut tx,
        NewTrack {
            title: new_scrobble.track_title.clone(),
            artist_ids: track_artist_ids,
            length: None,
            album_id: album_id,
            album_track_number: Some(0), // TODO
        },
    )
    .await
    .with_context(|| format!("failed to insert track {}", new_scrobble.track_title))?;

    let stmt = tx
        .prepare(
            "
        INSERT INTO scrobble (utc_timestamp, track_id, album_id) VALUES (CURRENT_TIMESTAMP, $1, $2)
        ",
        )
        .await
        .with_context(|| "failed to prepare scrobble insert query")?;

    tx.execute(&stmt, &[&track_id, &album_id])
        .await
        .with_context(|| format!("failed to insert scrobble for track id {track_id}"))?;

    tx.commit().await?;

    Ok(())
}
