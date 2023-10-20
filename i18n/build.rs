use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=languages.json");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("languages.rs");

    let value = fs::read_to_string(Path::new("languages.json")).unwrap();
    let languages: Vec<Vec<String>> =
        serde_json::from_str::<Vec<Vec<String>>>(&value).unwrap();

    let mut languages_rs_result =
        format!("pub static LANGUAGES: [Language; {}] = [", languages.len());
    for language in languages.iter() {
        languages_rs_result.push_str(&format!(
            "Language {{id: langid!(\"{}\"), name: \"{}\"}},",
            language[0], language[1]
        ));
    }
    languages_rs_result.push_str("];");

    fs::write(dest_path, languages_rs_result).unwrap();
}
