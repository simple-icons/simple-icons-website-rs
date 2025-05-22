extern crate simple_icons_website_end2end_steps;

use cucumber::World;
use simple_icons_website_end2end_helpers::AppWorld;

#[tokio::main]
async fn main() {
    AppWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit("./features/app")
        .await;
}
