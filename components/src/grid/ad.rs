use leptos::prelude::*;

/// Carbon ads add grid item
///
/// This is the main source of income for the Simple Icons project
#[component]
pub fn CarbonAdsAdGridItem() -> impl IntoView {
    view! {
        <script
            async
            src="//cdn.carbonads.com/carbon.js?serve=CKYIPK7M&placement=simpleiconsorg"
            type="text/javascript"
        ></script>
    }
}
