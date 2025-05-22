use anyhow::{Ok, Result};
use cucumber::{then, when};
use simple_icons_website_end2end_helpers::AppWorld;
use thirtyfour::prelude::*;

#[when(regex = "I type \"([^\"]+)\" in the search input")]
async fn type_in_search_input(
    world: &mut AppWorld,
    term: String,
) -> Result<()> {
    let xpath = ".//input[@type=\"search\"]";
    let input = world.client().find(By::XPath(xpath)).await?;
    input.send_keys(term.as_str()).await?;
    Ok(())
}

#[then(regex = "I see the icon \"([^\"]+)\" first")]
async fn check_icon_first(world: &mut AppWorld, term: String) -> Result<()> {
    let icon_title = world
        .client()
        .find(By::Css("main > ul > li:first-of-type h2"))
        .await?
        .text()
        .await?;
    assert_eq!(icon_title, term);
    Ok(())
}
