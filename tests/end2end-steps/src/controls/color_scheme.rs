use anyhow::{Ok, Result};
use cucumber::{then, when};
use simple_icons_website_end2end_helpers::{capitalize, AppWorld};
use thirtyfour::prelude::*;

#[when(
    regex = "click on the (light|dark|system) button of the color scheme control"
)]
async fn click_color_scheme_button(
    world: &mut AppWorld,
    button: String,
) -> Result<()> {
    let title = format!("{} color scheme", capitalize(&button));
    let xpath = format!(".//button[@title=\"{title}\"]");
    let button = world.client().find(By::XPath(xpath.as_str())).await?;
    button.click().await?;
    Ok(())
}

#[then(regex = "the app background is (light|dark)")]
async fn check_app_background(
    world: &mut AppWorld,
    background: String,
) -> Result<()> {
    let body = world.client().find(By::Tag("body")).await?;
    let class = body.attr("class").await?;
    assert!(class.is_some());
    let class = class.unwrap();
    assert!(class.contains(&background));
    Ok(())
}

#[then("the app background is the system color scheme")]
async fn check_app_background_is_system_color_scheme(
    world: &mut AppWorld,
) -> Result<()> {
    let ret = world
        .client()
        .execute(
            "return window.matchMedia('(prefers-color-scheme: dark)').matches",
            vec![],
        )
        .await?;
    match ret.json() {
        serde_json::Value::Bool(is_dark) => {
            check_app_background(
                world,
                if *is_dark {
                    "dark".to_string()
                } else {
                    "light".to_string()
                },
            )
            .await
        }
        _ => unreachable!(),
    }
}
