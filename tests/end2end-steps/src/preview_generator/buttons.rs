use anyhow::{Ok, Result};
use cucumber::when;
use end2end_helpers::AppWorld;
use thirtyfour::prelude::*;

#[when(
    regex = r#"I upload the file "([^"]+)" by clicking the "Upload SVG" button"#
)]
async fn upload_file_by_clicking_on_upload_svg_button(
    world: &mut AppWorld,
    filepath: String,
) -> Result<()> {
    let input_id = "preview-upload-svg-button";
    let input = world.driver().find(By::Id(input_id)).await?;

    let rooted_path = std::env::current_dir()
        .unwrap()
        .join("../..")
        .join(filepath)
        .to_str()
        .unwrap()
        .to_string();
    let canonical_path = std::fs::canonicalize(&rooted_path).unwrap();
    let abs_path = canonical_path.to_str().unwrap();
    input.send_keys(abs_path).await?;
    Ok(())
}

#[when(
    regex = r#"I press the "([^"]+)" \+ "([^"]+)" keys, the event "([^"]+)" is executed on the element "([^"]+)""#
)]
async fn press_keys_combination_and_check_event_is_executed_on_element(
    world: &mut AppWorld,
    modifier: String,
    key: String,
    element_event: String,
    selector: String,
) -> Result<()> {
    let modifier_key = match modifier.to_lowercase().as_str() {
        "ctrl" | "control" => Key::Control,
        "shift" => Key::Shift,
        "alt" => Key::Alt,
        "meta" => Key::Meta,
        _ => panic!("Unknown modifier key: {modifier}"),
    };
    let thirtyfour_key: TypingData = match key.to_lowercase().as_str() {
        "arrowup" => Key::Up.into(),
        "arrowdown" => Key::Down.into(),
        "arrowleft" => Key::Left.into(),
        "arrowright" => Key::Right.into(),
        k if k.len() == 1 => k.chars().next().unwrap().into(),
        _ => panic!("Unknown key: {key}"),
    };

    let driver = world.driver();

    driver
        .execute(
            r#"
                window.elementEventExecuted = false;
                const element = document.querySelector(arguments[0]);
                const previousElementEvent = element[arguments[1]];
                element[arguments[1]] = (event) => {
                    window.elementEventExecuted = true;
                    this.apply(previousElementEvent, event);
                };
            "#,
            vec![
                serde_json::Value::String(selector.clone()),
                serde_json::Value::String(element_event.clone()),
            ],
        )
        .await?;

    let body = driver.find(By::Tag("body")).await?;
    let actions = driver.action_chain();
    actions
        .key_down(modifier_key.clone())
        .send_keys_to_element(&body, thirtyfour_key)
        .key_up(modifier_key)
        .perform()
        .await?;

    let script_ret = driver
        .execute(
            r#"
                const result = window.elementEventExecuted;
                delete window.elementEventExecuted;
                return result;
            "#,
            vec![],
        )
        .await?;
    let clicked = match script_ret.json() {
        serde_json::Value::Bool(value) => *value,
        _ => unreachable!(),
    };

    assert!(
        clicked,
        "The event {element_event} has not been executed on the element {selector}"
    );
    Ok(())
}
