use anyhow::{Context, Result};
use deadpool_postgres::GenericClient;

use crate::db::{
    artist::insert_artist,
    track::{NewTrack, insert_track},
};

// TODO: album support
pub struct NewScrobble {
    pub track_title: String,
    pub track_artists: Vec<String>,
}

pub async fn insert_scrobble<C: GenericClient>(
    conn: &mut C,
    new_scrobble: &NewScrobble,
) -> Result<()> {
    println!(
        "Inserting scrobble: {} - {:?}",
        new_scrobble.track_title, new_scrobble.track_artists
    );

    let mut tx = conn
        .transaction()
        .await
        .with_context(|| "failed to begin transaction")?;

    let mut artists_ids: Vec<i32> = vec![];

    for artist in new_scrobble.track_artists.iter() {
        let id = insert_artist(&tx, artist)
            .await
            .with_context(|| format!("failed to insert artist {artist}"))?;
        artists_ids.push(id);
    }

    let track_id = insert_track(
        &mut tx,
        NewTrack {
            title: new_scrobble.track_title.clone(),
            artists_ids: artists_ids,
            length: None,
            album_id: None,
            album_track_number: None,
        },
    )
    .await
    .with_context(|| format!("failed to insert track {}", new_scrobble.track_title))?;

    let stmt = tx
        .prepare(
            "
        INSERT INTO scrobble (utc_timestamp, track_id) VALUES (CURRENT_TIMESTAMP, $1)
        ",
        )
        .await
        .with_context(|| "failed to prepare scrobble insert query")?;

    tx.execute(&stmt, &[&track_id])
        .await
        .with_context(|| format!("failed to insert scrobble for track id {track_id}"))?;

    tx.commit().await?;

    Ok(())
}
