use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use serde_json::Value;

//const BAD_TYPE_MSG: &str = "recieved value in an incorrect type";

#[derive(Debug, Clone)]
pub(crate) struct YoutubeConfig {
    pub locale: String, //String,
    pub logged_in: bool, //bool,
    pub visitor_data: String, //String,
    pub innertube_context_client_name: i64, //i64,
    pub device: String, //String,
    pub page_cl: i64, //i64,
    pub page_build_label: String, //String,
    pub innertube_api_version: String, //String,
    pub innertube_api_key: String, //String,
    pub innertube_client_name: String, //String,
    pub innertube_client_version: String, //String,
    pub gl: String, //String,
    pub hl: String, //String,
}

impl YoutubeConfig {
    pub(crate) fn new(full_config: &HashMap<String, Value>) -> Result<Self, ConfigParseError> {
        Ok(Self {
            locale: str_from_full(full_config, "LOCALE")?,
            logged_in: bool_from_full(full_config, "LOGGED_IN")?,
            visitor_data: str_from_full(full_config, "VISITOR_DATA")?,
            innertube_context_client_name: i64_from_full(full_config, "INNERTUBE_CONTEXT_CLIENT_NAME")?,
            device: str_from_full(full_config, "DEVICE")?,
            page_cl: i64_from_full(full_config, "PAGE_CL")?,
            page_build_label: str_from_full(full_config, "PAGE_BUILD_LABEL")?,
            innertube_api_version: str_from_full(full_config, "INNERTUBE_API_VERSION")?,
            innertube_api_key: str_from_full(full_config, "INNERTUBE_API_KEY")?,
            innertube_client_name: str_from_full(full_config, "INNERTUBE_CLIENT_NAME")?,
            innertube_client_version: str_from_full(full_config, "INNERTUBE_CLIENT_VERSION")?,
            gl: str_from_full(full_config, "GL")?,
            hl: str_from_full(full_config, "HL")?,
        })
    }
}

fn str_from_full(full_config: &HashMap<String, Value>, key: &str) -> Result<String, ConfigParseError> {
    match full_config.get(key) {
        Some(it) => match it {
            Value::String(it) => Ok(it.clone()),
            _ => Err(ConfigParseError::BadConfigParameter { key: key.to_string(), value: it.clone() }),
        },
        None => Err(ConfigParseError::MissingConfigParameter { key: key.to_string() }),
    }
}

fn i64_from_full(full_config: &HashMap<String, Value>, key: &str) -> Result<i64, ConfigParseError> {
    match full_config.get(key) {
        Some(it) => it.as_i64().ok_or(ConfigParseError::BadConfigParameter { key: key.to_string(), value: it.clone() }),
        None => Err(ConfigParseError::MissingConfigParameter { key: key.to_string() }),
    }
}

fn bool_from_full(full_config: &HashMap<String, Value>, key: &str) -> Result<bool, ConfigParseError> {
    match full_config.get(key) {
        Some(it) => it.as_bool().ok_or(ConfigParseError::BadConfigParameter { key: key.to_string(), value: it.clone() }),
        None => Err(ConfigParseError::MissingConfigParameter { key: key.to_string() }),
    }
}

#[derive(Debug)]
pub enum ConfigParseError {
    BadConfigParameter {
        key: String,
        value: Value,
    },
    MissingConfigParameter {
        key: String
    },
}

impl Display for ConfigParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigParseError::BadConfigParameter { key, value } => 
                write!(f, "config parameter {key} has a bad value: {value}"),
            ConfigParseError::MissingConfigParameter { key } =>
                write!(f, "config is missing a parameter {key}"),
        }
    }
}

impl Error for ConfigParseError {}