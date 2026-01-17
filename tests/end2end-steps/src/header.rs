use anyhow::{Ok, Result};
use cucumber::{then, when};
use end2end_helpers::{AppWorld, TouchesViewport};
use std::time::Duration;
use thirtyfour::{prelude::*, stringmatch::StringMatch};

#[then("the header touches the viewport")]
async fn header_touches_viewport(world: &mut AppWorld) -> Result<()> {
    let header = world.driver().find(By::Tag("header")).await?;
    let touches_viewport = world.element_touches_viewport(&header).await?;
    assert!(touches_viewport, "The header not touches viewport");
    Ok(())
}

#[then(regex = r#"^the title of the header is "(.+)""#)]
async fn check_header_title(world: &mut AppWorld, title: String) -> Result<()> {
    let header_title = world
        .driver()
        .query(By::Css("header > div > a"))
        .and_displayed()
        .first()
        .await?;
    let text = header_title.text().await?;
    assert_eq!(text, title);
    Ok(())
}

#[then(regex = r#"^the description of the header includes "(.+)""#)]
async fn check_header_description(
    world: &mut AppWorld,
    title: String,
) -> Result<()> {
    world
        .driver()
        .query(By::Css("header > div > p"))
        .first()
        .await?
        .wait_until()
        .has_text(StringMatch::new(&title).partial())
        .await?;
    Ok(())
}

#[then(regex = r"the app is in (English|Spanish)")]
async fn check_app_language(
    world: &mut AppWorld,
    language: String,
) -> Result<()> {
    let expected_description: &'static str = if language == "English" {
        "SVG icons for popular brands"
    } else {
        "iconos SVG para marcas populares"
    };
    check_header_description(world, expected_description.to_string()).await
}

#[when("I click on the language selector")]
async fn click_language_selector_button(world: &mut AppWorld) -> Result<()> {
    world
        .driver()
        .find(By::Css("header > nav > ul > li:last-of-type"))
        .await?
        .click()
        .await?;
    Ok(())
}

#[then("I see the language selector")]
async fn check_language_selector_modal(world: &mut AppWorld) -> Result<()> {
    world
        .driver()
        .query(By::Css(".language-selector"))
        .and_displayed()
        .all_from_selector_required()
        .await?;
    Ok(())
}

#[then("I don't see the language selector")]
async fn check_not_language_selector_modal(world: &mut AppWorld) -> Result<()> {
    world
        .driver()
        .query(By::Css(".language-selector"))
        .wait(Duration::from_millis(200), Duration::from_millis(10))
        .not_exists()
        .await?;
    Ok(())
}

#[then(regex = "I select the language ([\\S]+)")]
async fn select_language(
    world: &mut AppWorld,
    language_name: String,
) -> Result<()> {
    let xpath = format!(
        r#".//ul[@class="language-selector"]//li[text()={language_name}]"#
    );
    world
        .driver()
        .query(By::XPath(&xpath))
        .and_displayed()
        .first()
        .await?
        .click()
        .await?;
    Ok(())
}

#[when("I click on the extensions button")]
async fn click_extensions_button(world: &mut AppWorld) -> Result<()> {
    world
        .driver()
        .find(By::Css("header > nav > ul > li:nth-last-of-type(2)"))
        .await?
        .click()
        .await?;
    Ok(())
}

#[then("I see the extensions tables")]
async fn check_extensions_tables(world: &mut AppWorld) -> Result<()> {
    let tables = world
        .driver()
        .query(By::Css(".third-party-extensions"))
        .and_displayed()
        .all_from_selector_required()
        .await?;
    assert_eq!(tables.len(), 2, "There should be 2 extensions tables");
    Ok(())
}

#[then(
    regex = r#"I see an extension with the name "([^"]+)" and author "([^"]+)"#
)]
async fn check_extension_in_table(
    world: &mut AppWorld,
    name: String,
    author: String,
) -> Result<()> {
    // XPath que busca una fila <tr> que tenga:
    // 1️⃣ un <span> exactamente igual al nombre
    // 2️⃣ un <a> (autor) exactamente igual al author
    let xpath = format!(
        "//table[contains(@class, 'third-party-extensions')]//tr[
            .//span[text() = '{}'] and
            .//td/a[text() = '{}']
        ]",
        name, author
    );

    _ = world.driver().find(By::XPath(&xpath)).await?;

    Ok(())
}
