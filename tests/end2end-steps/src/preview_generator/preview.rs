use anyhow::{Ok, Result};
use base64::{
    Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD,
};
use cucumber::then;
use end2end_helpers::AppWorld;
use end2end_helpers::{equality_predicate, starts_with_predicate};
use std::time::Duration;
use thirtyfour::prelude::*;

#[then(
    regex = r#"the (title|filename|brand|color) in the preview is "([^"]+)""#
)]
async fn check_preview_data(
    world: &mut AppWorld,
    subject: String,
    value: String,
) -> Result<()> {
    let selector = match subject.as_str() {
        "title" => ".preview-figure > svg > g > text:nth-child(1)",
        "filename" => ".preview-figure > svg > g > text:nth-child(2)",
        "brand" => ".preview-figure > svg > g > text:nth-child(3)",
        "color" => ".preview-figure > svg > g > text:nth-child(4)",
        _ => unreachable!(),
    };
    let found = world
        .driver()
        .query(By::Css(selector))
        .wait(Duration::from_secs(6), Duration::from_millis(50))
        .with_filter(move |element: thirtyfour::WebElement| {
            let value = value.clone();
            async move {
                let text = element.inner_html().await;
                if let std::result::Result::Ok(text) = text {
                    return std::result::Result::Ok(text == value);
                }
                std::result::Result::Ok(false)
            }
        })
        .exists()
        .await?;
    assert!(found);
    Ok(())
}

#[then(regex = "the background color of the preview is (#[0-9a-fA-F]{3,6})")]
async fn check_preview_background_color(
    world: &mut AppWorld,
    color: String,
) -> Result<()> {
    let selector = ".preview-figure > svg > rect:nth-child(1)";
    let found = world
        .driver()
        .query(By::Css(selector))
        .wait(Duration::from_secs(6), Duration::from_millis(50))
        .with_filter(move |element: thirtyfour::WebElement| {
            let color = color.clone();
            async move {
                let fill = element.attr("fill").await;
                if let std::result::Result::Ok(Some(fill)) = fill {
                    return std::result::Result::Ok(fill == color);
                }
                std::result::Result::Ok(false)
            }
        })
        .exists()
        .await?;
    assert!(
        found,
        "The preview does not have the expected background color"
    );
    Ok(())
}

#[then(regex = r#"the SVG paths of the preview (start with|are) "([^"]+)""#)]
async fn check_preview_svg_paths(
    world: &mut AppWorld,
    mode: String,
    value: String,
) -> Result<()> {
    let client = world.driver().clone();

    let found = world
        .driver()
        .query(By::Css(".preview-figure > svg"))
        .wait(Duration::from_secs(6), Duration::from_millis(50))
        .with_filter(move |_| {
            let value = value.clone();
            let client = client.clone();
            let mode = mode.clone();
            async move {
                let paths_elements = client
                    .find_all(By::Css(".preview-figure > svg > svg > path"))
                    .await;
                if let std::result::Result::Ok(paths_elements) = paths_elements
                {
                    let mut paths = vec![];
                    for path_element in &paths_elements {
                        if let std::result::Result::Ok(Some(d)) =
                            path_element.attr("d").await
                        {
                            paths.push(d);
                        }
                    }
                    let predicate_fn = match mode.as_str() {
                        "are" => equality_predicate,
                        _ => starts_with_predicate,
                    };
                    let result = paths.len() == 4
                        && paths.iter().all(|d| predicate_fn(d, &value));
                    std::result::Result::Ok(result)
                } else {
                    std::result::Result::Ok(false)
                }
            }
        })
        .exists()
        .await?;
    assert!(found, "The SVG paths were not found as expected");
    Ok(())
}

#[then(regex = "the color of the badges in the preview is (#[0-9a-fA-F]{3,6})")]
async fn check_preview_badges_color(
    world: &mut AppWorld,
    color: String,
) -> Result<()> {
    let selector = ".preview-figure > svg > g:nth-child(7)";
    let found = world
        .driver()
        .query(By::Css(selector))
        .wait(Duration::from_secs(6), Duration::from_millis(50))
        .with_filter(move |element: thirtyfour::WebElement| {
            let expected_color = color.clone();
            async move {
                // flat + plastic + for the badge
                let background_badges =
                    element.find_all(By::Css("g > svg > g > rect")).await;
                let mut colors = vec![];
                if let std::result::Result::Ok(badges) = background_badges {
                    for badge in &badges {
                        if let std::result::Result::Ok(Some(fill)) =
                            badge.attr("fill").await
                        {
                            let found_color = fill.to_uppercase();
                            if found_color == expected_color {
                                colors.push(found_color);
                            }
                        }
                    }
                }

                let social_badges =
                    element.find_all(By::Css("g > svg > image")).await;
                if let std::result::Result::Ok(badges) = social_badges {
                    for badge in &badges {
                        if let std::result::Result::Ok(Some(href)) =
                            badge.attr("href").await
                        {
                            let base64_data =
                                href.split(',').nth(1).unwrap_or("");
                            let decoded_data =
                                BASE64_STANDARD.decode(base64_data);
                            if let std::result::Result::Ok(data) = decoded_data
                            {
                                let decoded_as_str =
                                    String::from_utf8_lossy(&data);
                                let maybe_fill = decoded_as_str
                                    .split_once("fill=\"")
                                    .and_then(|(_, rest)| {
                                        rest.split_once('"')
                                            .map(|(fill, _)| fill)
                                    });
                                if let Some(fill) = maybe_fill {
                                    let found_color = fill.to_uppercase();
                                    if found_color == expected_color {
                                        colors.push(found_color);
                                    }
                                }
                            }
                        }
                    }
                }

                std::result::Result::Ok(
                    colors.len() == 8
                        && colors.iter().all(|c| c == &expected_color),
                )
            }
        })
        .exists()
        .await?;
    assert!(found, "The badges with the expected color were not found");
    Ok(())
}

#[then(
    regex = r#"the logo SVG paths of the badges in the preview (start with|are) "([^"]+)""#
)]
async fn check_preview_badges_logo(
    world: &mut AppWorld,
    mode: String,
    expected_svg_path: String,
) -> Result<()> {
    let selector = ".preview-figure > svg > g:nth-child(7)";
    let found = world
        .driver()
        .query(By::Css(selector))
        .wait(Duration::from_secs(6), Duration::from_millis(50))
        .with_filter(move |element: thirtyfour::WebElement| {
            let expected_svg_path = expected_svg_path.clone();
            let mode = mode.clone();
            async move {
                let badge_images =
                    element.find_all(By::Css("g > svg image")).await;
                if let std::result::Result::Ok(badge_images) = badge_images {
                    let mut paths = vec![];

                    let predicate_fn = match mode.as_str() {
                        "are" => equality_predicate,
                        _ => starts_with_predicate,
                    };
                    for badge_image in &badge_images {
                        if let std::result::Result::Ok(Some(href)) =
                            badge_image.attr("href").await
                        {
                            let base64_data =
                                href.split(',').nth(1).unwrap_or("");
                            let decoded_data =
                                BASE64_STANDARD.decode(base64_data);
                            if let std::result::Result::Ok(data) = decoded_data
                            {
                                let decoded_as_str =
                                    String::from_utf8_lossy(&data);
                                let maybe_d = decoded_as_str
                                    .split_once("d=\"")
                                    .and_then(|(_, rest)| {
                                        rest.split_once('"').map(|(d, _)| d)
                                    });
                                if let Some(d) = maybe_d {
                                    paths.push(d.to_string());
                                }
                            }
                        }
                    }

                    let result = paths.len() == 8
                        && paths
                            .iter()
                            .all(|d| predicate_fn(d, &expected_svg_path));
                    return std::result::Result::Ok(result);
                }

                std::result::Result::Ok(false)
            }
        })
        .exists()
        .await?;
    assert!(found);
    Ok(())
}
