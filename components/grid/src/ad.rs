use leptos::prelude::*;
use leptos_meta::Link;
use simple_icons_website_ids::Ids;

/// Carbon ads add grid item
///
/// This is the main source of income for the Simple Icons project
#[component]
pub fn CarbonAdsAdGridItem() -> impl IntoView {
    view! {
        <Link rel="preconnect" href="https://cdn.carbonads.com" />
        <script
            id=Ids::CarbonadsJs
            async
            src="//cdn.carbonads.com/carbon.js?serve=CKYIPK7M&placement=simpleiconsorg"
            type="text/javascript"
        ></script>
    }
}
