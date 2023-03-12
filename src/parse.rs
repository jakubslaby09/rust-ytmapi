mod pointers;

use std::{fmt::Display, error::Error};

use serde_json::Value;

use crate::Client;

use self::pointers::*;

#[derive(Debug, Clone)]
pub struct Artist {
    pub name: String,
    pub description: Option<String>,
    pub albums: Vec<Product>,
    pub singles: Vec<Product>,
    //views: &'a str,
    //thumbnails: &'a str,
}

impl Artist {
    pub(crate) fn parse(res: Value) -> Result<Self, ResponseParseError> {
        Ok(Artist {
            name: string_from_json(&res, ARTIST_NAME)?,
            description: res.pointer(ARTIST_DESCIPTION).and_then(
                |it| Some(it.as_str()?.to_string())
            ),
            albums: iter_from_json(&res, ARTIST_ALBUMS)?.filter_map(|item| -> Option<Product> {
                Some(Product {
                    name: item.pointer(ARTIST_PRODUCT_NAME)?.as_str()?.to_string(),
                    browse_id: item.pointer(ARTIST_PRODUCT_ID)?.as_str()?.to_string(),
                    year: item.pointer(ARTIST_ALBUM_YEAR)?.as_str()?.to_string(),
                })
            }).collect(),
            singles: iter_from_json(&res, ARTIST_SINGLES)?.filter_map(|item| -> Option<Product> {
                Some(Product {
                    name: item.pointer(ARTIST_PRODUCT_NAME)?.as_str()?.to_string(),
                    browse_id: item.pointer(ARTIST_PRODUCT_ID)?.as_str()?.to_string(),
                    year: item.pointer(ARTIST_SINGLE_YEAR)?.as_str()?.to_string(),
                })
            }).collect(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub browse_id: String,
    pub year: String,
}

impl Product {
    pub async fn request(self: &Self, client: &Client) -> Option<Album> {
        client.get_album(&self.browse_id).await
    }
}

#[derive(Debug, Clone)]
pub struct Album {
    pub name: String,
    // pub browse_id: String,
    pub year: String,
    pub tracks: Vec<Track>,
}

impl Album {
    pub(crate) fn parse(res: Value) -> Option<Self> {
        Some(
            Album {
                name: res.pointer(ALBUM_NAME)?.as_str()?.to_string(),
                year: res.pointer(ALBUM_YEAR)?.as_str()?.to_string(),
                tracks: res.pointer(ALBUM_TRACKS)?
                .as_array()?.into_iter().enumerate().filter_map(|(track_num, item)| -> Option<Track> {
                    Some(Track {
                        name: item.pointer(ALBUM_TRACK_NAME)?.as_str()?.to_string(),
                        video_id: item.pointer(ALBUM_TRACK_ID)?.as_str()?.to_string(),
                        track_num: track_num + 1,
                    })
                }).collect(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub name: String,
    pub video_id: String,
    pub track_num: usize,
}

#[derive(Debug, Clone)]
pub struct ArtistSearchResult {
    pub name: String,
    pub subs: String,
    pub browse_id: String,
}

impl ArtistSearchResult {
    pub(crate) fn parse(res: Value) -> Option<Vec<Self>> {
        Some(
            res.pointer(SEARCHED_ARTISTS)?
            .as_array()?.into_iter().filter_map(|item| -> Option<Self> {
                Some(Self {
                    name: item.pointer(SEARCHED_ARTIST_NAME)?.as_str()?.to_string(),
                    subs: item.pointer(SEARCHED_ARTIST_SUBS)?.as_str()?.to_string(),
                    browse_id: item.pointer(SEARCHED_ARTIST_ID)?.as_str()?.to_string(),
                })
            }).collect(),
        )
    }
    
    pub async fn request(self: &Self, client: &Client) -> Result<Artist, Box<dyn Error>> {
        client.get_artist(&self.browse_id).await
    }
}

fn value_from_json<'a>(value: &'a Value, pointer: &str) -> Result<&'a Value, ResponseParseError> {
    value.pointer(pointer)
    .ok_or(ResponseParseError::MissingValue(pointer.to_string()))
}

fn string_from_json(value: &Value, pointer: &str) -> Result<String, ResponseParseError>{
    match value_from_json(value, pointer)?.as_str() {
        Some(it) => Ok(it.to_string()),
        None => Err(ResponseParseError::BadValue(pointer.to_string(), value.clone())),
    }
}

fn iter_from_json<'a>(value: &'a Value, pointer: &str) -> Result<std::slice::Iter<'a, Value>, ResponseParseError> {
    match value_from_json(value, pointer)?.as_array() {
        Some(it) => Ok(it.iter()),
        None => Err(ResponseParseError::BadValue(pointer.to_string(), value.clone())),
    }
}

#[derive(Debug)]
pub enum ResponseParseError {
    MissingValue(String),
    BadValue(String, Value),
}

impl Display for ResponseParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseParseError::MissingValue(path) => write!(f, "Response is missing a value at {path}"),
            ResponseParseError::BadValue(path, value) => write!(f, "Response has a bad value at {path}: {value}"),
        }
    }
}
impl Error for ResponseParseError {}