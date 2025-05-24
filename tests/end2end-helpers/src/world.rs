#[cucumber_thirtyfour_worlder::worlder]
pub struct AppWorld;

/// Check wether an element is in the viewport
pub async fn element_touches_viewport(
    world: &AppWorld,
    element: &WebElement,
) -> Result<bool, WebDriverError> {
    let ret = world
        .driver()
        .execute(
            r#"
            const element = arguments[0];
            const box = element.getBoundingClientRect();
            return box.top >= 0 && box.left >= 0;
            "#,
            vec![element.to_json()?],
        )
        .await?;
    match ret.json() {
        serde_json::Value::Bool(value) => Ok(*value),
        _ => unreachable!(),
    }
}
