//! Track attributes returned by Spotify's internal audio-attributes service.

use serde::{Deserialize, Serialize};

use crate::Error;

/// Audio attributes for a track. These are Spotify's estimates, not values
/// calculated locally by librespot. Availability depends on Spotify's service.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AudioFeatures {
    /// Estimated tempo in beats per minute (BPM).
    pub tempo: f64,
    /// Pitch class: C = 0, C-sharp/D-flat = 1, ..., B = 11; -1 means unknown.
    pub key: i32,
    /// Minor = 0, major = 1.
    pub mode: i32,
    /// Estimated beats per bar.
    pub time_signature: Option<i32>,
    /// Overall loudness in decibels (dB).
    pub loudness: Option<f64>,
    /// Energy estimate, from 0.0 to 1.0.
    pub energy: Option<f64>,
    /// Danceability estimate, from 0.0 to 1.0.
    pub danceability: Option<f64>,
    /// Musical positivity estimate, from 0.0 to 1.0.
    pub valence: Option<f64>,
    /// Confidence that the track is acoustic, from 0.0 to 1.0.
    pub acousticness: Option<f64>,
    /// Likelihood that the track has no vocals, from 0.0 to 1.0.
    pub instrumentalness: Option<f64>,
    /// Presence of spoken words, from 0.0 to 1.0.
    pub speechiness: Option<f64>,
    /// Likelihood of a live performance, from 0.0 to 1.0.
    pub liveness: Option<f64>,
    /// Base62 track ID, when supplied by the service.
    pub id: Option<String>,
    pub uri: Option<String>,
    pub duration_ms: Option<u32>,
}

#[derive(Deserialize)]
struct AudioFeaturesResponse {
    audio_features: Vec<Option<AudioFeatures>>,
}

pub(crate) fn parse_batch(
    body: &[u8],
    expected_count: usize,
) -> Result<Vec<Option<AudioFeatures>>, Error> {
    let response: AudioFeaturesResponse = serde_json::from_slice(body)?;
    if response.audio_features.len() != expected_count {
        return Err(Error::failed_precondition(
            "audio features response does not match the requested track count",
        ));
    }
    Ok(response.audio_features)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_attributes_and_ignores_new_fields() {
        let features: AudioFeatures = serde_json::from_str(
            r#"{"tempo":123.456,"key":1,"mode":0,"time_signature":4,
                "loudness":-8.5,"energy":0.8,"danceability":0.7,"valence":0.6,
                "acousticness":0.1,"instrumentalness":0.2,"speechiness":0.05,
                "liveness":0.3,"duration_ms":210000,"id":"track-id",
                "uri":"spotify:track:track-id","future_attribute":true}"#,
        )
        .unwrap();
        assert_eq!(features.tempo, 123.456);
        assert_eq!((features.key, features.mode), (1, 0));
        assert_eq!(features.time_signature, Some(4));
        assert_eq!(features.loudness, Some(-8.5));
        assert_eq!(features.energy, Some(0.8));
        assert_eq!(features.danceability, Some(0.7));
        assert_eq!(features.valence, Some(0.6));
        assert_eq!(features.acousticness, Some(0.1));
        assert_eq!(features.instrumentalness, Some(0.2));
        assert_eq!(features.speechiness, Some(0.05));
        assert_eq!(features.liveness, Some(0.3));
        assert_eq!(features.duration_ms, Some(210000));
        assert_eq!(features.id.as_deref(), Some("track-id"));
        assert_eq!(features.uri.as_deref(), Some("spotify:track:track-id"));
    }

    #[test]
    fn preserves_unknown_key_and_absent_optional_values() {
        let features: AudioFeatures =
            serde_json::from_str(r#"{"tempo":120,"key":-1,"mode":1,"energy":null}"#).unwrap();
        assert_eq!(features.key, -1);
        assert_eq!(features.energy, None);
        assert_eq!(features.time_signature, None);
    }

    #[test]
    fn preserves_batch_positions_and_duplicates() {
        let body = br#"{"audio_features":[{"tempo":120,"key":0,"mode":1},null,
            {"tempo":120,"key":0,"mode":1}]}"#;
        let features = parse_batch(body, 3).unwrap();
        assert!(features[0].is_some());
        assert!(features[1].is_none());
        assert_eq!(features[0], features[2]);
        assert!(parse_batch(body, 2).is_err());
    }

    #[test]
    fn rejects_errors_and_malformed_responses() {
        for body in [
            r#"{"error":{"status":403,"message":"Forbidden"}}"#,
            r#"{"tempo":null,"key":0,"mode":1}"#,
            r#"{"tempo":120,"mode":1}"#,
            "null",
            "not json",
        ] {
            assert!(serde_json::from_str::<AudioFeatures>(body).is_err());
        }
        assert!(parse_batch(br#"{"audio_features":[{"error":{}}]}"#, 1).is_err());
        assert!(parse_batch(br#"{}"#, 0).is_err());
    }
}
