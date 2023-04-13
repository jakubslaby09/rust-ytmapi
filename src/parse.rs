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
                    name: string_from_json(&item, ARTIST_PRODUCT_NAME).ok()?,
                    browse_id: string_from_json(&item, ARTIST_PRODUCT_ID).ok()?,
                    year: string_from_json(&item, ARTIST_ALBUM_YEAR).ok()?,
                })
            }).collect(),
            singles: iter_from_json(&res, ARTIST_SINGLES)?.filter_map(|item| -> Option<Product> {
                Some(Product {
                    name: string_from_json(&item, ARTIST_PRODUCT_NAME).ok()?,
                    browse_id: string_from_json(&item, ARTIST_PRODUCT_ID).ok()?,
                    year: string_from_json(&item, ARTIST_SINGLE_YEAR).ok()?,
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
    pub async fn request(self: &Self, client: &Client) -> Result<Album, Box<dyn Error>> {
        client.get_album(&self.browse_id).await
    }
}

#[derive(Debug, Clone)]
pub struct Album {
    pub name: String,
    // pub browse_id: String,
    pub year: String,
    pub tracks: Vec<Track>,
    pub thumbnails: Vec<Thumbnail>,
}

impl Album {
    pub(crate) fn parse(res: Value) -> Result<Self, ResponseParseError> {
        Ok(Album {
            name: string_from_json(&res, ALBUM_NAME)?,
            year: string_from_json(&res, ALBUM_YEAR)?,
            tracks: iter_from_json(&res, ALBUM_TRACKS)?
            .enumerate().filter_map(|(track_num, item)| -> Option<Track> {
                Some(Track {
                    name: string_from_json(&item, ALBUM_TRACK_NAME).ok()?,
                    video_id: string_from_json(&item, ALBUM_TRACK_ID).ok()?,
                    track_num: track_num + 1,
                })
            }).collect(),
            thumbnails: iter_from_json(&res, ALBUM_THUMBS)?
            .filter_map(|thumbnail| -> Option<Thumbnail> {
                Some(Thumbnail {
                    url: string_from_json(&thumbnail, ALBUM_THUMB_URL).ok()?,
                    size: (
                        value_from_json(&thumbnail, ALBUM_THUMB_WIDTH).ok()?.as_u64()? as usize,
                        value_from_json(&thumbnail, ALBUM_THUMB_HEIGHT).ok()?.as_u64()? as usize,
                    ),
                })
            }).collect(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub name: String,
    pub video_id: String,
    pub track_num: usize,
}

#[derive(Debug, Clone)]
pub struct Thumbnail {
    pub url: String,
    pub size: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct ArtistSearchResult {
    pub name: String,
    pub subs: String,
    pub browse_id: String,
}

impl ArtistSearchResult {
    pub(crate) fn parse(res: Value) -> Result<Vec<Self>, ResponseParseError> {
        Ok(iter_from_json(&res, SEARCHED_ARTISTS)?.filter_map(|item| -> Option<Self> {
            Some(Self {
                name: string_from_json(&item, SEARCHED_ARTIST_NAME).ok()?,
                subs: string_from_json(&item, SEARCHED_ARTIST_SUBS).ok()?,
                browse_id: string_from_json(&item, SEARCHED_ARTIST_ID).ok()?,
            })
        }).collect())
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