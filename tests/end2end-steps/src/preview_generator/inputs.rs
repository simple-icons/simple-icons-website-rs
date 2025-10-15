use anyhow::{Ok, Result};
use cucumber::{then, when};
use end2end_helpers::AppWorld;
use thirtyfour::prelude::*;

#[when(regex = "I click the (brand|color|path) input")]
async fn click_input(world: &mut AppWorld, input: String) -> Result<()> {
    let id = format!("preview-{input}");
    world.driver().find(By::Id(id)).await?.click().await?;
    Ok(())
}

#[then("I can see some brand suggestions")]
async fn check_brand_suggestions(world: &mut AppWorld) -> Result<()> {
    let suggestions = world
        .driver()
        .query(By::Css(".preview-brand-suggestions > li"))
        .wait(
            std::time::Duration::from_secs(6),
            std::time::Duration::from_millis(10),
        )
        .and_displayed()
        .all_from_selector()
        .await?;
    assert!(
        !suggestions.is_empty(),
        "No brand suggestions are displayed"
    );
    Ok(())
}

#[when("I click on the first brand suggestion")]
async fn click_first_brand_suggestion(world: &mut AppWorld) -> Result<()> {
    let first_suggestion = world
        .driver()
        .query(By::Css(".preview-brand-suggestions > li"))
        .wait(
            std::time::Duration::from_secs(6),
            std::time::Duration::from_millis(10),
        )
        .and_displayed()
        .first()
        .await?;
    first_suggestion.click().await?;
    Ok(())
}

#[then(
    regex = r#"the (brand|color|path) input value (is|starts with) "([^"]+)""#
)]
async fn check_input_value(
    world: &mut AppWorld,
    input: String,
    equality_mode: String,
    brand: String,
) -> Result<()> {
    let id = format!("preview-{input}");
    let brand_input = world.driver().find(By::Id(id)).await?;
    let value = brand_input.attr("value").await?;
    if equality_mode == "is" {
        assert_eq!(value.as_deref(), Some(brand.as_str()));
    } else {
        assert!(
            value
                .as_deref()
                .is_some_and(|v| v.starts_with(brand.as_str())),
            "The input value does not start with the expected value"
        );
    }
    Ok(())
}

#[when(regex = r#"I type "([^"]+)" in the (brand|color|path) input"#)]
async fn type_in_input(
    world: &mut AppWorld,
    value: String,
    input: String,
) -> Result<()> {
    let id = format!("preview-{input}");
    let color_input = world.driver().find(By::Id(id)).await?;
    color_input.clear().await?;
    color_input.send_keys(value).await?;
    Ok(())
}
