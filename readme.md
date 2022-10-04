# A library for fetching Youtube Music metadata

currently, these features are included:
- searching for an artist by name
- getting an artist's singles and albums
- listing tracks and their ids from an album

this library was inspired by [youtube-music-api](https://github.com/emresenyuva/youtube-music-api) written in javascript

## Code example

```rust
    // Request configs from youtube music
    let client = Client::init().await.unwrap();
    
    // Get an album from the first artist in the search results
    let search_results = client.search_artists(QUERY).await.unwrap();
    let artist = client.get_artist(&search_results[0].browse_id).await.unwrap();
    let album = client.get_album(&artist.albums[0].browse_id).await.unwrap();
    
    println!("first album: {:#?}", album);
```