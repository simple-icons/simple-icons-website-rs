/// High level step definitions for end-to-end tests.
use super::{AppWorld, Waiter};
use anyhow::{Ok, Result};
use cucumber::{then, when};
use std::time::Duration;
use thirtyfour::prelude::*;

#[when(regex = r#"I click on the element "([^"]+)"$"#)]
async fn click_element(world: &mut AppWorld, selector: String) -> Result<()> {
    let driver = world.driver();
    driver
        .find(By::Css(selector.clone()))
        .await?
        .click()
        .await?;
    Ok(())
}

#[then(regex = r#"a file named "([^"]+)" is downloaded within (\d+) seconds"#)]
async fn check_file_is_downloaded(
    world: &mut AppWorld,
    file_name: String,
    seconds: String,
) -> Result<()> {
    let message = format!(
        "The file {file_name} has not been downloaded within {seconds} seconds"
    );
    let timeout = Duration::from_secs(seconds.parse::<u64>()?);
    let interval = Duration::from_millis(50);

    let directory = world.downloads_dir();
    let download_path = std::path::Path::new(directory);
    let file_path = download_path.join(file_name);

    let condition = move || {
        let file_path = file_path.clone();
        async move {
            Ok(file_path.exists()).map_err(|e| {
                thirtyfour::error::WebDriverError::UnknownError(
                    thirtyfour::error::WebDriverErrorInfo::new(e.to_string()),
                )
            })
        }
    };

    Waiter::new(timeout, interval, message)
        .until(|| [&condition])
        .await?;

    Ok(())
}
