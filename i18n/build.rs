use std::env;
use std::fs;
use std::path::Path;

static DEFAULT_LANGUAGE: &str = "en-US";

fn main() {
    println!("cargo:rerun-if-changed=languages.json");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("languages.rs");

    let value = fs::read_to_string(Path::new("languages.json")).unwrap();
    let mut languages: Vec<Vec<String>> =
        serde_json::from_str::<Vec<Vec<String>>>(&value).unwrap();
    // Sort languages and put default the first
    languages.sort_by(|a, b| {
        if a[0] == DEFAULT_LANGUAGE {
            std::cmp::Ordering::Less
        } else if b[0] == DEFAULT_LANGUAGE {
            std::cmp::Ordering::Greater
        } else {
            a[1].cmp(&b[1])
        }
    });

    let mut languages_rs_result = format!(
        "pub static LANGUAGES: [crate::Language; {}] = [",
        languages.len()
    );
    for language in languages.iter() {
        languages_rs_result.push_str(&format!(
            "crate::Language {{id: ::unic_langid::langid!(\"{}\"), name: \"{}\"}},",
            language[0], language[1]
        ));
    }
    languages_rs_result.push_str("];");

    fs::write(dest_path, languages_rs_result).unwrap();
}
