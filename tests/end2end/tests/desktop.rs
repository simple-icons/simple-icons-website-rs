extern crate simple_icons_website_end2end_steps;

use cucumber::World;

#[tokio::main]
async fn main() {
    simple_icons_website_end2end_helpers::AppWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit("./features/desktop")
        .await
}
