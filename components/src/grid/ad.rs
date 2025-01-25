use leptos::prelude::*;

/// Carbon ads add grid item
///
/// This is the main source of income for the Simple Icons project
#[component]
pub fn CarbonAdsAdGridItem() -> impl IntoView {
    view! {
        <style>r#".layout-compact #carbonads { grid-row: -3/-1; padding-top: 15%; }"#</style>
        <script
            async
            src="//cdn.carbonads.com/carbon.js?serve=CKYIPK7M&placement=simpleiconsorg"
            type="text/javascript"
        ></script>
    }
}
