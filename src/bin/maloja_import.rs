use anyhow::Result;
use chlorine::db;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MalojaAlbum {
    artists: Option<Vec<String>>,
    albumtitle: String,
}

#[derive(Deserialize, Debug)]
struct MalojaTrack {
    artists: Vec<String>,
    title: String,
    album: Option<MalojaAlbum>,
    length: Option<u16>,
}

#[derive(Deserialize, Debug)]
struct MalojaScrobble {
    time: Option<i64>,
    track: MalojaTrack,
    duration: Option<u16>,
    origin: Option<String>,
}

#[derive(Deserialize, Debug)]
struct MalojaExportFile {
    scrobbles: Vec<MalojaScrobble>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = std::env::args()
        .nth(1)
        .expect("missing maloja export file path argument");
    let file = std::fs::read_to_string(file_path)?;
    // TODO: deserialize with streaming
    let maloja_export: MalojaExportFile = serde_json::from_str(&file)?;

    let config = chlorine::config::AppConfig::from_env()?;

    // TODO: use a single connection
    let pool = db::create_pool(&config)?;

    let mut conn = pool.get().await.inspect_err(|err| eprintln!("{:?}", err))?;

    let mut tx = conn.transaction().await?;

    // TODO: cache artist name -> artist_id
    // TODO: cache album name and artists -> album_id
    // TODO: cache track name and artists -> track_id
    for scrobble in maloja_export.scrobbles {
        let utc_timestamp = if let Some(time) = scrobble.time {
            Some(jiff::Timestamp::new(time, 0)?)
        } else {
            None
        };

        db::scrobble::insert_scrobble(
            &mut tx,
            &db::scrobble::NewScrobble {
                utc_timestamp: utc_timestamp,
                track_title: scrobble.track.title,
                track_artists: scrobble.track.artists,
                album_title: None,
                album_artists: None,
            },
        )
        .await?;
    }

    tx.commit().await?;

    pool.close();

    Ok(())
}
