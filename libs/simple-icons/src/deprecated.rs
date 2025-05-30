use std::env;
use std::fs;
use std::path::Path;

/// Deprecated icons for next versions of Simple Icons
#[derive(Clone, Debug)]
pub struct IconDeprecation {
    pub slug: String,
    pub at_version: String,
    pub milestone_number: u64,
    pub milestone_due_on: String,
    pub pull_request_number: u64,
    /// If new_slug is Some, the icon was renamed
    pub new_slug: Option<String>,
}

/// Implements `PartialEq` for `IconDeprecation`.
///
/// Sometimes the simple-icons maintainers make a mistake where they open
/// a second pull request to remove an icon which was previously opened,
/// creating a duplicate entry in the list of deprecated icons that only
/// can be seen as a duplicate by their slug.
///
/// See https://github.com/simple-icons/simple-icons/pull/11844
impl PartialEq for IconDeprecation {
    fn eq(&self, other: &Self) -> bool {
        self.slug == other.slug
    }
}

/**
 * Get all the icons that will be removed in the next major versions
 * ordered by version.
 **/
pub fn fetch_deprecated_simple_icons() -> Vec<IconDeprecation> {
    let tmp_file_name = "simple-icons-deprecated.json";
    let tmp_file_path = Path::new(&env::temp_dir()).join(tmp_file_name);
    assert!(
        tmp_file_path.exists(),
        "Run `cargo make` to execute the script fetch-deprecated-icons.rs and build."
    );

    let resp: serde_json::Value = serde_json::from_str::<serde_json::Value>(
        &fs::read_to_string(&tmp_file_path).unwrap(),
    )
    .unwrap();

    if let Some(message) = resp.get("message") {
        fs::remove_file(tmp_file_path).unwrap();
        panic!("Error retrieving data from GITHUB Graphql API: {message}");
    }

    let milestones_data = resp
        .get("data")
        .unwrap()
        .get("repository")
        .unwrap()
        .get("milestones")
        .unwrap()
        .get("nodes")
        .unwrap()
        .as_array()
        .unwrap();

    let mut deprecated_icons: Vec<IconDeprecation> = Vec::new();

    for milestone_data in milestones_data.iter() {
        let title = milestone_data.get("title").unwrap().as_str().unwrap();
        let at_version = title.replace('v', "");
        let milestone_number =
            milestone_data.get("number").unwrap().as_u64().unwrap();
        let milestone_due_on =
            milestone_data.get("dueOn").unwrap().as_str().unwrap();

        let pull_requests_data = milestone_data
            .get("pullRequests")
            .unwrap()
            .get("nodes")
            .unwrap()
            .as_array()
            .unwrap();
        for pull_request_data in pull_requests_data.iter() {
            let pull_request_number =
                pull_request_data.get("number").unwrap().as_u64().unwrap();

            let files_data = pull_request_data
                .get("files")
                .unwrap()
                .get("edges")
                .unwrap()
                .as_array()
                .unwrap();

            let mut new_slug: Option<String> = None;
            let mut contains_deleted_icon = false;
            for file_data in files_data.iter() {
                let path = file_data
                    .get("node")
                    .unwrap()
                    .get("path")
                    .unwrap()
                    .as_str()
                    .unwrap_or_default();
                if !path.starts_with("icons/") || !path.ends_with(".svg") {
                    continue;
                }

                let change_type =
                    file_data.get("node").unwrap().get("changeType").unwrap();
                if change_type == "DELETED" {
                    contains_deleted_icon = true;
                } else if change_type == "ADDED" {
                    new_slug =
                        Some(path.replace("icons/", "").replace(".svg", ""));
                }
            }

            for file_data in files_data.iter() {
                let path = file_data
                    .get("node")
                    .unwrap()
                    .get("path")
                    .unwrap()
                    .as_str()
                    .unwrap_or_default();
                if !path.starts_with("icons/") || !path.ends_with(".svg") {
                    continue;
                }

                let change_type =
                    file_data.get("node").unwrap().get("changeType").unwrap();
                if change_type == "ADDED"
                    && contains_deleted_icon
                    && new_slug.is_some()
                {
                    continue;
                }

                if change_type != "DELETED" {
                    continue;
                }

                let slug =
                    path.to_string().replace("icons/", "").replace(".svg", "");

                let deprecated_icon = IconDeprecation {
                    slug,
                    at_version: at_version.to_string(),
                    milestone_number,
                    milestone_due_on: milestone_due_on.to_string(),
                    pull_request_number,
                    new_slug: if contains_deleted_icon && new_slug.is_some() {
                        new_slug.clone()
                    } else {
                        None
                    },
                };

                if !deprecated_icons.contains(&deprecated_icon) {
                    deprecated_icons.push(deprecated_icon);
                }
            }
        }
    }

    deprecated_icons
}
