#[allow(clippy::single_component_path_imports)]
use cucumber;
use std::future::Future;
use thirtyfour::{self, WebElement, error::WebDriverError};

#[cucumber_thirtyfour_worlder::worlder(
    cucumber = cucumber,
    thirtyfour = thirtyfour,
    check_concurrency_cli_option_when_firefox = true,
)]
pub struct AppWorld;

pub trait TouchesViewport {
    /// Check if the element is in the viewport
    fn element_touches_viewport(
        &self,
        element: &WebElement,
    ) -> impl Future<Output = Result<bool, WebDriverError>>;
}

impl TouchesViewport for AppWorld {
    fn element_touches_viewport(
        &self,
        element: &WebElement,
    ) -> impl Future<Output = Result<bool, WebDriverError>> {
        let execute_future = self.driver().execute(
            r#"
            const element = arguments[0];
            const box = element.getBoundingClientRect();
            return box.top >= 0 && box.left >= 0;
            "#,
            vec![
                element
                    .to_json()
                    .expect("Failed to convert element to JSON"),
            ],
        );
        async {
            let ret = execute_future.await?;
            match ret.json() {
                serde_json::Value::Bool(value) => Ok(*value),
                _ => unreachable!(),
            }
        }
    }
}
