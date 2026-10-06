//! Retrieve audio features for one or more tracks.
//!
//! Usage: SPOTIFY_ACCESS_TOKEN=... cargo run --example audio_features -- TRACK_ID [TRACK_ID ...]
//! Obtain a user access token using the OAuth flow in examples/README.md.

use std::env;

use librespot::core::{Error, Session, SessionConfig, SpotifyId, authentication::Credentials};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let ids = env::args()
        .skip(1)
        .map(|id| SpotifyId::from_base62(&id))
        .collect::<Result<Vec<_>, _>>()?;
    if ids.is_empty() {
        eprintln!(
            "Usage: SPOTIFY_ACCESS_TOKEN=... cargo run --example audio_features -- TRACK_ID [TRACK_ID ...]"
        );
        return Ok(());
    }

    let access_token = env::var("SPOTIFY_ACCESS_TOKEN")
        .map_err(|_| Error::invalid_argument("SPOTIFY_ACCESS_TOKEN must be set"))?;
    let session = Session::new(SessionConfig::default(), None);
    session
        .connect(Credentials::with_access_token(access_token), false)
        .await?;

    if ids.len() == 1 {
        let features = session.spclient().get_audio_features(&ids[0]).await?;
        println!("{features:#?}");
    } else {
        let features = session.spclient().get_audio_features_batch(&ids).await?;
        println!("{features:#?}");
    }
    Ok(())
}
