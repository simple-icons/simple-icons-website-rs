extern crate simple_icons_website_end2end_steps;

use simple_icons_website_end2end_helpers::AppWorld;

#[tokio::main]
async fn main() {
    AppWorld::run_features("./features/desktop").await;
}
