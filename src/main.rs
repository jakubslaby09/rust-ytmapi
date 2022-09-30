use std::collections::HashMap;
use parse::{Artist, Album};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value, Map};

mod config;
mod parse;
use config::YoutubeConfig;

use crate::requests::{create_api_request, endpoint_context};

mod requests;

pub(crate) const BASE_URL: &str = "https://music.youtube.com/";
const BROWSE_ID: &str = "UC0725SlKeEA-U4YOzrlYGWg";

#[tokio::main]
async fn main() {
    println!("init...");
    let client = Client::init().await.unwrap();
    let artist = client.get_artist(BROWSE_ID).await.unwrap();
    let album = client.get_album(artist.albums[0].browse_id.as_str()).await.unwrap();
    
    // let album = client.get_album("MPREb_1GgxHArHaap").await.unwrap();
    // write("res.json", serde_json::to_string(&album).unwrap());
    println!("albums: {:#?}", album);
    
}

struct Client {
    config: YoutubeConfig
}

impl Client {
    async fn get_artist(self: &Self, browse_id: &str) -> Option<Artist> {
        let res = match create_api_request(
            &self.config, "browse", endpoint_context("ARTIST", browse_id)
        ).await {
            Ok(it) => it,
            Err(_) => return None,
        };

        Some(Artist::parse(res)?)
    }
    
    async fn get_album(self: &Self, browse_id: &str) -> Option<Album> {
        let res = match create_api_request(
            &self.config, "browse", endpoint_context("ALBUM", browse_id)
        ).await {
            Ok(it) => it,
            Err(_) => return None,
        };

        // Some(res)

        Some(Album::parse(res)?)
    }
    
    async fn init() -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::new();
        let headers_map = Vec::from([
            ("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:104.0) Gecko/20100101 Firefox/104.0"),
            ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*//*;q=0.8"),
            ("Accept-Language", "en-US,en;q=0.5"),
            ("Upgrade-Insecure-Requests", "1"),
            ("Sec-Fetch-Dest", "document"),
            ("Sec-Fetch-Mode", "navigate"),
            ("Sec-Fetch-Site", "none"),
            ("Sec-Fetch-User", "?1"),
        ]);
        
        let mut headers = HeaderMap::new();
        for header in headers_map {
            headers.insert(header.0, HeaderValue::from_str(header.1).unwrap());
        }

        let response = client
            .get(BASE_URL)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        //write("res.txt", &res).await?;

        let mut full_config = HashMap::new();
        response.as_str().split("ytcfg.set(").into_iter().skip(1).for_each(|s: &str| {
            let text = s.split(");").nth(0).unwrap();//.replace("'", "\"");
            let json_res: Result<Map<String, Value>, serde_json::Error> = serde_json::from_str(text);
            if let Ok(json) = json_res {
                for prop in json.into_iter() {
                    full_config.insert(prop.0, prop.1);
                }
            }
        });
        let config = YoutubeConfig::new(&full_config);
        // println!("\n\n{:#?}", config);
        
        Ok(Client {
            config,
        })
    }
}

