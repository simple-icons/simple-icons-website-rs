//! Cucumber app world builder

use std::borrow::Cow;

use cucumber::World;
use thirtyfour::{
    error::WebDriverError, BrowserCapabilitiesHelper, DesiredCapabilities,
    WebDriver, WebElement,
};

const SUPPORTED_BROWSERS: [&str; 2] = ["chrome", "firefox"];

fn readable_supported_browsers() -> String {
    SUPPORTED_BROWSERS.join(", ")
}

#[derive(Debug)]
pub struct AppWorldClientOptions {
    host: &'static str,
    driver_url: &'static str,
    browser: Cow<'static, str>,
}

impl Default for AppWorldClientOptions {
    fn default() -> Self {
        if std::env::var("BROWSER").is_err() {
            let supported_browsers = readable_supported_browsers();
            panic!(
                "Please set the BROWSER environment variable to the browser \
                 you want to use. Supported browsers: {supported_browsers}"
            );
        }
        let browser = std::env::var("BROWSER").unwrap();
        Self::check_browser(&browser);

        Self {
            host: "http://127.0.0.1:8080",
            driver_url: "http://localhost:4444",
            browser: Cow::Owned(browser),
        }
    }
}

impl AppWorldClientOptions {
    #[must_use]
    pub fn host(&self) -> &str {
        self.host
    }

    pub fn with_host(mut self, host: &'static str) -> Self {
        self.host = host;
        self
    }

    #[must_use]
    pub fn driver_url(&self) -> &str {
        self.driver_url
    }

    pub fn with_driver_url(mut self, driver_url: &'static str) -> Self {
        self.driver_url = driver_url;
        self
    }

    fn check_browser(browser: &str) {
        if !SUPPORTED_BROWSERS.contains(&browser) {
            let supported_browsers = readable_supported_browsers();
            panic!(
                "Browser '{browser}' is not supported. \
                 Supported browsers: {supported_browsers}"
            );
        }
    }

    #[must_use]
    pub fn browser(&self) -> &str {
        self.browser.as_ref()
    }

    pub fn with_browser(mut self, browser: &'static str) -> Self {
        Self::check_browser(browser);
        self.browser = Cow::Borrowed(browser);
        self
    }
}

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct AppWorld {
    client: WebDriver,
    client_options: AppWorldClientOptions,
}

impl AppWorld {
    #[must_use]
    pub fn client(&self) -> &WebDriver {
        &self.client
    }

    #[must_use]
    pub fn client_options(&self) -> &AppWorldClientOptions {
        &self.client_options
    }

    pub async fn new() -> Self {
        let client_options = AppWorldClientOptions::default();
        Self::new_with_client_options(client_options).await
    }

    pub async fn new_with_client_options(
        client_options: AppWorldClientOptions,
    ) -> Self {
        let client = Self::build_client(&client_options).await;
        Self {
            client,
            client_options,
        }
    }

    async fn build_client(client_options: &AppWorldClientOptions) -> WebDriver {
        let driver_url = client_options.driver_url;
        if client_options.browser == "chrome" {
            let mut caps = DesiredCapabilities::chrome();
            let opts = vec!["--no-sandbox"];
            caps.insert_browser_option("args", opts)
                .unwrap_or_else(|err| {
                    panic!("Failed to set Chrome options: {err}");
                });
            WebDriver::new(driver_url, caps)
                .await
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to create WebDriver for Chrome: {err}. \
                        Make sure that chromedriver server is running at {driver_url}",
                    )
                })
        } else {
            WebDriver::new(driver_url, DesiredCapabilities::firefox()).await.unwrap_or_else(|err| {
                panic!(
                    "Failed to create WebDriver for Firefox: {err}. \
                    Make sure that geckodriver server is running at {driver_url}",
                )
            })
        }
    }

    // Helpers for tests

    /// Navigate to the given path inside the host
    pub async fn goto_path(&self, path: &str) -> Result<&Self, WebDriverError> {
        let url = format!("{}{}", self.client_options().host(), path);
        if let Err(err) = self.client.goto(&url).await {
            Err(err)
        } else {
            Ok(self)
        }
    }

    /// Check wether an element is in the viewport
    pub async fn element_touches_viewport(
        &self,
        element: &WebElement,
    ) -> Result<bool, WebDriverError> {
        let ret = self
            .client
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
}
