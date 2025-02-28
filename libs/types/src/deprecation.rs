/// Static deprecation data for a simple icon
#[derive(Clone)]
pub struct IconDeprecation {
    pub at_version: &'static str,
    pub milestone_number: u64,
    pub milestone_due_on: &'static str,
    pub pull_request_number: u64,
    pub renamed: bool,
}

impl IconDeprecation {
    pub fn get_milestone_url(&self) -> String {
        format!(
            "https://github.com/simple-icons/simple-icons/milestone/{}",
            self.milestone_number
        )
    }

    pub fn get_pull_request_url(&self) -> String {
        format!(
            "https://github.com/simple-icons/simple-icons/pull/{}",
            self.pull_request_number
        )
    }
}
