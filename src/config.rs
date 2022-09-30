use std::collections::HashMap;

use serde_json::Value;

//const BAD_TYPE_MSG: &str = "recieved value in an incorrect type";

#[derive(Debug, Clone)]
pub(crate) struct YoutubeConfig {
    pub locale: String, //String,
    pub logged_in: String, //bool,
    pub visitor_data: String, //String,
    pub innertube_context_client_name: String, //i64,
    pub device: String, //String,
    pub page_cl: String, //i64,
    pub page_build_label: String, //String,
    pub innertube_api_version: String, //String,
    pub innertube_api_key: String, //String,
    pub innertube_client_name: String, //String,
    pub innertube_client_version: String, //String,
    pub gl: String, //String,
    pub hl: String, //String,
}

impl YoutubeConfig {
    pub(crate) fn new(full_config: &HashMap<String, Value>) -> Self {
        Self {
            locale: from_full(full_config, "LOCALE"),
            logged_in: from_full(full_config, "LOGGED_IN"),
            visitor_data: from_full(full_config, "VISITOR_DATA"),
            innertube_context_client_name: from_full(full_config, "INNERTUBE_CONTEXT_CLIENT_NAME"),
            device: from_full(full_config, "DEVICE"),
            page_cl: from_full(full_config, "PAGE_CL"),
            page_build_label: from_full(full_config, "PAGE_BUILD_LABEL"),
            innertube_api_version: from_full(full_config, "INNERTUBE_API_VERSION"),
            innertube_api_key: from_full(full_config, "INNERTUBE_API_KEY"),
            innertube_client_name: from_full(full_config, "INNERTUBE_CLIENT_NAME"),
            innertube_client_version: from_full(full_config, "INNERTUBE_CLIENT_VERSION"),
            gl: from_full(full_config, "GL"),
            hl: from_full(full_config, "HL"),
        }
    }
}

fn from_full(full_config: &HashMap<String, Value>, key: &str) -> String {
    match full_config.get(key) {
        Some(value) => match value.as_str() {
            Some(string) => string.to_owned(),
            None => match value.as_i64() {
                Some(int) => int.to_string(),
                None => match value.as_bool() {
                    Some(b) => b.to_string(),
                    None => panic!("Recieved unexpexted type of value {:?}", value),
                },
            },
        },
        None => "".to_owned(),
    }
}