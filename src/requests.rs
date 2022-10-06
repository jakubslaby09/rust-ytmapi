use reqwest::{header::{HeaderMap, HeaderValue}, Url};
use serde_json::{json, Value, Map};

use crate::{config::YoutubeConfig, BASE_URL};

pub(crate) async fn create_api_request(_config: &YoutubeConfig, endpoint_name: &str, input_variables: Map<String, Value>) -> Result<Value, reqwest::Error> {
    let mut url = Url::parse(BASE_URL).expect("invalid base url");
    url.set_path(format!("youtubei/{}/{}", _config.innertube_api_version, endpoint_name).as_str());
    
    let config = _config.clone();
    let headers_map = Vec::from([
        ("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:104.0) Gecko/20100101 Firefox/104.0"),
        ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*//*;q=0.8"),
        ("Accept-Language", "en-US,en;q=0.5"),
        ("Upgrade-Insecure-Requests", "1"),
        ("Sec-Fetch-Dest", "document"),
        ("Sec-Fetch-Mode", "navigate"),
        ("Sec-Fetch-Site", "none"),
        ("Sec-Fetch-User", "?1"),
        ("x-origin", BASE_URL),
        ("X-Goog-Visitor-Id", &config.visitor_data),
        ("X-YouTube-Client-Name", &config.innertube_context_client_name),
        ("X-YouTube-Client-Version", &config.innertube_client_version),
        ("X-YouTube-Device", &config.device),
        ("X-YouTube-Page-CL", &config.page_cl),
        ("X-YouTube-Page-Label", &config.page_build_label),
        //"X-YouTube-Utc-Offset", String(-new Date().getTimezoneOffset()),
        //"X-YouTube-Time-Zone", new Intl.DateTimeFormat().resolvedOptions().timeZone
    ]);

    let mut headers = HeaderMap::new();
    for header in headers_map {
        headers.insert(header.0, HeaderValue::from_str(header.1).unwrap());
    }

    let mut body = Map::new();
    for item in api_context(_config) {
        body.insert(item.0, item.1);
    }
    for item in input_variables {
        body.insert(item.0, item.1);
    }

    let res = reqwest::Client::new().post(url)
        .headers(headers)
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await?
        .json()
        .await?;
    
    Ok(res)
}

pub(crate) fn endpoint_context(type_name: &str, browse_id: &str) -> Map<String, Value> {
    json!({
        "browseEndpointContextSupportedConfigs": {
            "browseEndpointContextMusicConfig": {
                "pageType": format!("MUSIC_PAGE_TYPE_{}", type_name)
            }
        },
        "browseId": browse_id
    }).as_object().unwrap().to_owned()
}

pub(crate) fn api_context(config: &YoutubeConfig) -> Map<String, Value> {
    json!({
        "context": {
            "capabilities": {},
            "client": {
                "clientName": config.innertube_client_name,
                "clientVersion": config.innertube_client_version,
                "experimentIds": [],
                "experimentsToken": "",
                "gl": config.gl,
                "hl": config.hl,
                "locationInfo": {
                    "locationPermissionAuthorizationStatus": "LOCATION_PERMISSION_AUTHORIZATION_STATUS_UNSUPPORTED",
                },
                "musicAppInfo": {
                    "musicActivityMasterSwitch": "MUSIC_ACTIVITY_MASTER_SWITCH_INDETERMINATE",
                    "musicLocationMasterSwitch": "MUSIC_LOCATION_MASTER_SWITCH_INDETERMINATE",
                    "pwaInstallabilityStatus": "PWA_INSTALLABILITY_STATUS_UNKNOWN",
                },
                //"utcOffsetMinutes": -new Date().getTimezoneOffset(),
            },
            "request": {
                "internalExperimentFlags": [{
                        "key": "force_music_enable_outertube_tastebuilder_browse",
                        "value": "true",
                    },
                    {
                        "key": "force_music_enable_outertube_playlist_detail_browse",
                        "value": "true",
                    },
                    {
                        "key": "force_music_enable_outertube_search_suggestions",
                        "value": "true",
                    },
                ],
                "sessionIndex": {},
            },
            "user": {
                "enableSafetyMode": false,
            },
        }
    }).as_object().unwrap().to_owned()
}