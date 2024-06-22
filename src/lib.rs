use std::{collections::HashMap, error::Error};
use serde_json::{Value, Map, json};

mod config;
mod parse;

use config::YoutubeConfig;
use crate::requests::{create_api_request, endpoint_context};
pub use crate::parse::*;

mod requests;

pub(crate) const BASE_URL: &str = "https://music.youtube.com/";

// TODO: add much more tests
#[tokio::main]
#[test]
async fn main() {
    let client = Client::init().await.unwrap();
    
    println!("searching for artist");
    let results = client.search_artists("Rammstein").await.unwrap();

    println!("requesting: {}", results[0].name);
    let artist = client.get_artist(&results[0].browse_id).await.unwrap();
    
    println!("requesting album: {}", artist.albums[0].name);
    let album = client.get_album(&artist.albums[0].browse_id).await.unwrap();
    
    println!("album: {:#?}", album);
}

#[derive(Clone, Debug)]
pub struct Client {
    pub config: YoutubeConfig
}

impl Client {
    /// Search an artist by their channel's name
    ///
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = youtube_music::Client::init().await.unwrap();
    ///     let results = client.search_artists("Rammstein").await.unwrap();
    ///     if let Some(first_result) = results.into_iter().next() {
    ///         dbg!(client.get_artist(&first_result.browse_id).await.unwrap());
    ///     }
    /// }
    /// ```
    pub async fn get_artist(self: &Self, browse_id: &str) -> Result<Artist, Box<dyn Error>> {
        let res = create_api_request(
            &self.config, "browse", endpoint_context("ARTIST", browse_id)
        ).await?;

        // std::fs::write("res.art.json", res.to_string()).unwrap();
        Ok(Artist::parse(res)?)
    }
    
    /// Search an artist by their channel's name
    ///
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = youtube_music::Client::init().await.unwrap();
    ///     let results = client.search_artists("Rammstein").await.unwrap();
    ///     if let Some(first_result) = results.into_iter().next() {
    ///         let first_artist = client.get_artist(&first_result.browse_id).await.unwrap();
    ///         if let Some(first_album) = first_artist.albums.into_iter().next() {
    ///             dbg!(client.get_artist(&first_album.browse_id).await.unwrap());
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_album(self: &Self, browse_id: &str) -> Result<Album, Box<dyn Error>> {
        let res = create_api_request(
            &self.config, "browse", endpoint_context("ALBUM", browse_id)
        ).await?;

        // std::fs::write("res.alb.json", res.to_string()).unwrap();
        Ok(Album::parse(res)?)
    }

    /// Search an artist by their channel's name
    ///
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = youtube_music::Client::init().await.unwrap();
    ///     let results = client.search_artists("Rammstein").await.unwrap();
    ///     if let Some(first_result) = results.into_iter().next() {
    ///         dbg!(first_result);
    ///     }
    /// }
    /// ```
    pub async fn search_artists(self: &Self, query: &str) -> Result<Vec<ArtistSearchResult>, Box<dyn Error>> {
        let body_vars = json!({
            "params": "EgWKAQIgAWoKEAkQChADEAUQBA%3D%3D",
            "query": query,
           }).as_object().unwrap().to_owned();
        let res = create_api_request(&self.config, "search", body_vars)
            .await?;
        // std::fs::write("res.search.json", res.to_string()).unwrap();
        Ok(ArtistSearchResult::parse(res)?)
    }
    
    /// Request configs from Youtube music
    /// 
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = youtube_music::Client::init().await.unwrap();
    ///     
    ///     dbg!(client);
    /// }
    /// ```
    pub async fn init() -> Result<Client, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let response = client
            .get(BASE_URL)
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:104.0) Gecko/20100101 Firefox/104.0")
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*//*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Upgrade-Insecure-Requests", "1")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "none")
            .header("Sec-Fetch-User", "?1")
            .send()
            .await?
            .text()
            .await?;

        let mut full_config = HashMap::new();
        for config_unenclosed in response.as_str().split("ytcfg.set(").into_iter().skip(1) {
            let config_text = config_unenclosed.split_once(");").ok_or(
                ResponseParseError::UnclosedConfig(config_unenclosed.to_string())
            )?.0;
            if let Ok(config) = serde_json::from_str::<Map<String, Value>>(config_text) {
                for prop in config.into_iter() {
                    full_config.insert(prop.0, prop.1);
                }
            }
        };
        let config = YoutubeConfig::new(&full_config)?;
        
        Ok(Client {
            config,
        })
    }
}