use anyhow::{Ok, Result};
use cucumber::{then, when};
use end2end_helpers::AppWorld;
use thirtyfour::prelude::*;

#[when(regex = "I type \"([^\"]+)\" in the search input")]
async fn type_in_search_input(
    world: &mut AppWorld,
    term: String,
) -> Result<()> {
    let xpath = ".//input[@type=\"search\"]";
    let input = world.driver().find(By::XPath(xpath)).await?;
    input.send_keys(term.as_str()).await?;
    Ok(())
}

#[then(regex = "I see the icon \"([^\"]+)\" first")]
async fn check_icon_first(world: &mut AppWorld, term: String) -> Result<()> {
    let icon_title = world
        .driver()
        .find(By::Css("main > ul > li:first-of-type h2"))
        .await?
        .text()
        .await?;
    assert_eq!(icon_title, term);
    Ok(())
}
