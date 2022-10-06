# A library for fetching Youtube Music metadata

currently, these features are included:
- searching for an artist by name
- getting an artist's singles and albums
- listing tracks and their ids from an album

this library was inspired by [youtube-music-api](https://github.com/emresenyuva/youtube-music-api) written in javascript

## Code examples

```rust
    // Request configs from youtube music
    let client = Client::init().await.unwrap();
    
    // Get an album from the first artist in the search results
    let album = client.search_artists("Tři sestry").await.unwrap()
    [0].request(&client).await.unwrap()
    .albums[0].request(&client).await.unwrap();
    
    println!("first album: {:#?}", album);
```

```rust
    // Request configs from youtube music
    let client = Client::init().await.unwrap();
    
    // Get an album from the first artist in the search results

    // Search for an artist
    let search_results = client.search_artists(QUERY).await.unwrap();

    // Get the first result
    let artist = client.get_artist(&search_results[0].browse_id).await.unwrap();

    // Get it's first album
    let album = client.get_album(&artist.albums[0].browse_id).await.unwrap();
    
    println!("first album: {:#?}", album);
```