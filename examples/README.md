# Examples

This folder contains examples of how to use the `librespot` library for various purposes.

## How to run the examples

In general, to invoke an example, clone down the repo and use `cargo` as follows:

```
cargo run --example [filename]
```

in which `filename` is the file name of the example, for instance `get_token` or `play`.

### Acquiring an access token

Most examples require an access token as the first positional argument. **Note that an access token
gained by the client credentials flow will not work**. `librespot-oauth` provides a utility to 
acquire an access token using an OAuth flow, which will be able to run the examples. To invoke this, 
run:

```
cargo run --package librespot-oauth --example oauth_sync
```

A browser window will open and prompt you to authorize with Spotify. Once done, take the 
`access_token` property from the dumped object response and proceed to use it in examples. You may
find it convenient to save it in a shell variable like `$ACCESS_TOKEN`.

Once you have obtained the token you can proceed to run the example. Check each individual
file to see what arguments are expected. As a demonstration, here is how to invoke the `play` 
example to play a song -- the second argument is the URI of the track to play.

```
cargo run --example play "$ACCESS_TOKEN" 2WUy2Uywcj5cP0IXQagO3z
```

### Track audio features (BPM and key)

With an authenticated Premium session, call
`session.spclient().get_audio_features(&track_id).await?` to retrieve an
`AudioFeatures` value, or `get_audio_features_batch(&track_ids)` for multiple
tracks. Batch calls split input into groups of 100 and preserve `null` results as
`None`. Tempo is in BPM; key uses pitch classes (C = 0 through B = 11, -1 for
unknown); mode is 0 for minor or 1 for major. Other attributes are optional.

The `audio_features` example reads an access token from the environment and
accepts one or more base62 track IDs:

```sh
SPOTIFY_ACCESS_TOKEN="$ACCESS_TOKEN" cargo run --example audio_features -- 2WUy2Uywcj5cP0IXQagO3z
```

Use the OAuth flow described below to obtain a user access token. This uses
Spotify's internal `/audio-attributes/v1/audio-features` service, as used by
[spicetify-dj-info](https://github.com/L3-N0X/spicetify-dj-info/blob/main/src/api/metadata.mjs); availability and permissions may change independently of
librespot. Request errors are returned to the caller; no values are estimated
locally. See the feature request:
https://github.com/librespot-org/librespot/discussions/1780.
