use anyhow::{Ok, Result};
use cucumber::{then, when};
use end2end_helpers::{capitalize, AppWorld};
use thirtyfour::prelude::*;

#[then(regex = "the selected layout is (comfortable|compact)")]
async fn check_layout(world: &mut AppWorld, layout: String) -> Result<()> {
    let grid = world.driver().find(By::Css("main > ul")).await?;
    let class = grid
        .attr("class")
        .await
        .unwrap_or_default()
        .unwrap_or_default();

    // the class is only set if the layout is not default
    if layout == "compact" {
        assert!(
            class.contains("layout-compact") && !class.contains("comfortable")
        );
    } else {
        assert!(!class.contains("comfortable") && !class.contains("compact"));
    }

    Ok(())
}

#[when(
    regex = "click on the (comfortable|compact) button of the layout control"
)]
async fn click_layout_button(
    world: &mut AppWorld,
    layout: String,
) -> Result<()> {
    let xpath = format!(".//button[@title=\"{}\"]", capitalize(&layout));
    let button = world.driver().find(By::XPath(xpath.as_str())).await?;
    button.click().await?;

    Ok(())
}
