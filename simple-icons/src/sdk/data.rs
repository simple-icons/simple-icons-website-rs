// TODO: nanoserde giving clippy warning:
// 'this block may be rewritten with the `?` operator'
// #[derive(DeJson)]
//          ^^^^^^
#![allow(clippy::question_mark)]

use nanoserde::DeJson;
use std::collections::HashMap;
use std::fs;
use std::path;

#[derive(DeJson, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct SimpleIconDataLicense {
    #[nserde(rename = "type")]
    pub type_: String,
    pub url: Option<String>,
}

#[derive(DeJson, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct SimpleIconDataDuplicate {
    pub title: String,
    pub hex: Option<String>,
    pub guidelines: Option<String>,
    pub loc: Option<HashMap<String, String>>,
}

#[derive(DeJson, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct SimpleIconDataAliases {
    pub aka: Option<Vec<String>>,
    pub dup: Option<Vec<SimpleIconDataDuplicate>>,
    pub loc: Option<HashMap<String, String>>,
}

#[derive(DeJson)]
pub struct SimpleIconData {
    pub slug: Option<String>,
    pub title: String,
    pub hex: String,
    pub source: String,
    pub guidelines: Option<String>,
    pub license: Option<SimpleIconDataLicense>,
    pub aliases: Option<SimpleIconDataAliases>,
}

#[derive(DeJson)]
pub struct SimpleIconsData {
    pub icons: Vec<SimpleIconData>,
}

pub fn get_simple_icons_data() -> SimpleIconsData {
    let icons_data_file =
        path::Path::new("node_modules/simple-icons/_data/simple-icons.json");
    let icons_data_raw = fs::read_to_string(icons_data_file)
        .expect("Could not read simple-icons.json file");
    DeJson::deserialize_json(&icons_data_raw)
        .expect("JSON was not well-formatted")
}
