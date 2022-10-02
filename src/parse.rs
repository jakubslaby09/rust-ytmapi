use serde_json::Value;

#[derive(Debug)]
pub struct Artist {
    pub name: String,
    pub description: String,
    pub albums: Vec<Product>,
    pub singles: Vec<Product>,
    //views: &'a str,
    //thumbnails: &'a str,
}

impl Artist {
    pub(crate) fn parse(res: Value) -> Option<Self> {
        Some(Artist {
            name: res.pointer("/header/musicImmersiveHeaderRenderer/title/runs/0/text")?.as_str()?.to_string(),
            description: res.pointer("/header/musicImmersiveHeaderRenderer/description/runs/0/text")?.as_str()?.to_string(),
            albums: res.pointer("/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents/1/musicCarouselShelfRenderer/contents")?
            .as_array()?.into_iter().filter_map(|item| -> Option<Product> {
                Some(Product {
                    name: item.pointer("/musicTwoRowItemRenderer/title/runs/0/text")?.as_str()?.to_string(),
                    browse_id: item.pointer("/musicTwoRowItemRenderer/title/runs/0/navigationEndpoint/browseEndpoint/browseId")?.as_str()?.to_string(),
                    year: item.pointer("/musicTwoRowItemRenderer/subtitle/runs/2/text")?.as_str()?.to_string(),
                })
            }).collect(),
            singles: res.pointer("/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents/2/musicCarouselShelfRenderer/contents")?
            .as_array()?.into_iter().filter_map(|item| -> Option<Product> {
                Some(Product {
                    name: item.pointer("/musicTwoRowItemRenderer/title/runs/0/text")?.as_str()?.to_string(),
                    browse_id: item.pointer("/musicTwoRowItemRenderer/title/runs/0/navigationEndpoint/browseEndpoint/browseId")?.as_str()?.to_string(),
                    year: item.pointer("/musicTwoRowItemRenderer/subtitle/runs/0/text")?.as_str()?.to_string(),
                })
            }).collect(),
        })
    }
}

#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub browse_id: String,
    pub year: String,
}

#[derive(Debug)]
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
                name: res.pointer("/header/musicDetailHeaderRenderer/title/runs/0/text")?.as_str()?.to_string(),
                year: res.pointer("/header/musicDetailHeaderRenderer/subtitle/runs/4/text")?.as_str()?.to_string(),
                tracks: res.pointer("/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents/0/musicShelfRenderer/contents")?
                .as_array()?.into_iter().filter_map(|item| -> Option<Track> {
                    Some(Track {
                        name: item.pointer("/musicResponsiveListItemRenderer/flexColumns/0/musicResponsiveListItemFlexColumnRenderer/text/runs/0/text")?.as_str()?.to_string(),
                        video_id: item.pointer("/musicResponsiveListItemRenderer/flexColumns/0/musicResponsiveListItemFlexColumnRenderer/text/runs/0/navigationEndpoint/watchEndpoint/videoId")?.as_str()?.to_string(),
                    })
                }).collect(),
            }
        )
    }
}

#[derive(Debug)]
pub struct Track {
    pub name: String,
    pub video_id: String,
}

#[derive(Debug)]
pub struct ArtistSearchResult {
    pub name: String,
    pub subs: String,
    pub browse_id: String,
}

impl ArtistSearchResult {
    pub(crate) fn parse(res: Value) -> Option<Vec<Self>> {
        Some(
            res.pointer("/contents/tabbedSearchResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents/1/musicShelfRenderer/contents")?
            .as_array()?.into_iter().filter_map(|item| -> Option<Self> {
                Some(Self {
                    name: item.pointer("/musicResponsiveListItemRenderer/flexColumns/0/musicResponsiveListItemFlexColumnRenderer/text/runs/0/text")?.as_str()?.to_string(),
                    subs: item.pointer("/musicResponsiveListItemRenderer/flexColumns/1/musicResponsiveListItemFlexColumnRenderer/text/runs/2/text")?.as_str()?.to_string(),
                    browse_id: item.pointer("/musicResponsiveListItemRenderer/navigationEndpoint/browseEndpoint/browseId")?.as_str()?.to_string(),
                })
            }).collect(),
        )
    }
}