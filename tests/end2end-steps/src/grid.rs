use anyhow::{Ok, Result};
use core::str::FromStr;
use cucumber::{given, then, when};
use end2end_helpers::AppWorld;
use simple_icons_website_controls_layout_type::Layout;
use std::time::Duration;
use thirtyfour::prelude::*;

#[given("I see the grid")]
async fn grid_is_displayed(world: &mut AppWorld) -> Result<()> {
    let grid = world.driver().find(By::Css("main > ul")).await?;
    let displayed = grid.is_displayed().await?;
    assert!(displayed);
    Ok(())
}

#[then(
    regex = "the (comfortable|compact) number of icons per page have been loaded"
)]
async fn default_number_of_icons_per_page_loaded(
    world: &mut AppWorld,
    layout: String,
) -> Result<()> {
    number_of_icons_per_page_loaded(world, layout, "1".to_string()).await
}

#[then(
    regex = r"the (comfortable|compact) number of icons per page \* (\d+) have been loaded"
)]
async fn number_of_icons_per_page_loaded(
    world: &mut AppWorld,
    layout: String,
    multiplicator: String,
) -> Result<()> {
    let client = world.driver().clone();

    let expected_number_of_icons =
        Layout::from_str(&layout).unwrap().icons_per_page() as usize
            * multiplicator.parse::<usize>()?;

    let found = client
        .query(By::Css("main > ul"))
        .wait(Duration::from_secs(6), Duration::from_millis(50))
        .with_filter(move |_| {
            let client = client.clone();
            async move {
                let elements = client.find_all(By::Css("main > ul > li")).await;
                if let std::result::Result::Ok(elements) = elements {
                    return std::result::Result::Ok(
                        elements.len() == expected_number_of_icons,
                    );
                }
                std::result::Result::Ok(false)
            }
        })
        .exists()
        .await?;
    assert!(found);
    Ok(())
}

#[given("I scroll to the top")]
async fn scroll_to_top(world: &mut AppWorld) -> Result<()> {
    let header = world.driver().find(By::Tag("header")).await?;
    header.scroll_into_view().await?;
    Ok(())
}

#[when(r#"I click on the "Go to footer" button"#)]
async fn click_go_to_footer_button(world: &mut AppWorld) -> Result<()> {
    let button = world
        .driver()
        .find(By::ClassName("scroll-to-footer-button"))
        .await?;
    button.click().await?;
    Ok(())
}

#[when(r#"I click on the "Go to header" button"#)]
async fn click_go_to_header_button(world: &mut AppWorld) -> Result<()> {
    let button = world
        .driver()
        .find(By::ClassName("scroll-to-header-button"))
        .await?;
    button.click().await?;
    Ok(())
}

#[given(r#"the "Go to header" button does not exists"#)]
async fn go_to_header_button_does_not_exists(
    world: &mut AppWorld,
) -> Result<()> {
    let not_exists = world
        .driver()
        .query(By::ClassName("scroll-to-header-button"))
        .nowait()
        .not_exists()
        .await?;
    assert!(not_exists);
    Ok(())
}

#[then(r#"the "Load more icons" button does not exists"#)]
async fn load_more_icons_button_does_not_exists(
    world: &mut AppWorld,
) -> Result<()> {
    let not_exists = world
        .driver()
        .query(By::Css(".icons-loader > button"))
        .nowait()
        .not_exists()
        .await?;
    assert!(not_exists);
    Ok(())
}

#[then(r#"I see the "Load more icons" button"#)]
async fn load_more_icons_button_is_displayed(
    world: &mut AppWorld,
) -> Result<()> {
    let button = world
        .driver()
        .find(By::Css(".icons-loader > button"))
        .await?;
    let is_displayed = button.is_displayed().await?;
    assert!(is_displayed);
    Ok(())
}

#[when(r#"I click on the "Load more icons" button"#)]
async fn load_more_icons_button_click(world: &mut AppWorld) -> Result<()> {
    let button = world
        .driver()
        .find(By::Css(".icons-loader > button"))
        .await?;
    button.click().await?;
    Ok(())
}
