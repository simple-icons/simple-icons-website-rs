use anyhow::{Ok, Result};
use cucumber::{given, then, when};
use end2end_helpers::{AppWorld, TouchesViewport};
use std::time::Duration;
use thirtyfour::prelude::*;

#[given("I see the app")]
async fn open_the_app(world: &mut AppWorld) -> Result<()> {
    _ = world
        .goto_path("")
        .await?
        .driver()
        .query(By::Tag("header"))
        .wait(Duration::from_secs(60), Duration::from_millis(100))
        .and_displayed()
        .first()
        .await?;
    Ok(())
}

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
        .wait(Duration::from_secs(6), Duration::from_millis(10))
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
        .wait(Duration::from_millis(1000), Duration::from_millis(10))
        .with_filter(move |e: WebElement| {
            let title = title.clone();
            async move {
                let text = e.text().await;
                if let std::result::Result::Ok(text) = text {
                    return std::result::Result::Ok(
                        text.contains(title.as_str()),
                    );
                }
                std::result::Result::Ok(false)
            }
        })
        .first()
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
        .wait(Duration::from_millis(200), Duration::from_millis(10))
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
        .wait(Duration::from_millis(200), Duration::from_millis(10))
        .and_displayed()
        .first()
        .await?
        .click()
        .await?;
    Ok(())
}
