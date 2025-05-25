use anyhow::{Ok, Result};
use cucumber::{then, when};
use end2end_helpers::{AppWorld, TouchesViewport};
use thirtyfour::prelude::*;

#[when("I scroll to the footer")]
async fn scroll_to_footer(world: &mut AppWorld) -> Result<()> {
    let footer = world.driver().find(By::Tag("footer")).await?;
    footer.scroll_into_view().await?;
    Ok(())
}

#[then("the footer touches the viewport")]
async fn footer_touches_viewport(world: &mut AppWorld) -> Result<()> {
    let footer = world.driver().find(By::Tag("footer")).await?;
    let touches_viewport = world.element_touches_viewport(&footer).await?;
    assert!(touches_viewport, "The footer not touches viewport");
    Ok(())
}
