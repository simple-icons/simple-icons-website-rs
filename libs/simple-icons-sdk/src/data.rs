// TODO: nanoserde giving clippy warning:
// 'this block may be rewritten with the `?` operator'
// #[derive(DeJson)]
//          ^^^^^^
#![allow(clippy::question_mark)]

use nanoserde::DeJson;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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
    pub old: Option<Vec<String>>,
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

/// Extract the Simple Icons data file path from its package.json exports.
///
/// We could change its path. We've changed it in v15
/// (see https://github.com/simple-icons/simple-icons/pull/13316)
/// and maybe we'll do it again in the future.
fn get_simple_icons_data_file_path() -> PathBuf {
    let package_json_file = Path::new("node_modules")
        .join("simple-icons")
        .join("package.json");
    let package_json_raw = fs::read_to_string(&package_json_file)
        .expect("Could not read package.json file");
    let package_json: serde_json::Value =
        serde_json::from_str(&package_json_raw)
            .expect("JSON was not well-formatted");
    let exported_data_file_path = package_json
        .get("exports")
        .and_then(|exports| {
            exports
                .get("./icons.json")
                .and_then(|data| data.get("default"))
        })
        .expect("Exported data file path not found in `.exports.[./icons.json].default` of Simple Icons package.json")
        .as_str()
        .expect("Exported data file path is not a string");

    let icons_data_file = Path::new("node_modules")
        .join("simple-icons")
        .join(exported_data_file_path);
    if !icons_data_file.exists() {
        panic!(
            "Simple Icons data file does not exist at path: {}",
            icons_data_file.display()
        );
    }
    icons_data_file
}

pub fn get_simple_icons_data() -> Vec<SimpleIconData> {
    let icons_data_file = get_simple_icons_data_file_path();
    let icons_data_raw = fs::read_to_string(&icons_data_file)
        .expect("Could not read simple-icons.json file");
    DeJson::deserialize_json(&icons_data_raw)
        .expect("JSON was not well-formatted")
}
