use crate::header::HeaderStateSignal;
use i18n::{move_tr, tr};
use leptos::*;
use macros::get_number_of_icons;
use std::collections::HashMap;

/// Header title
///
/// It includes the title and the description
/// about what is Simple Icons shown below the title.
#[component]
pub fn HeaderTitle() -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>().unwrap().0;
    let description_html = move_tr!("site-description", &{
        let mut map = HashMap::new();
        map.insert(
            "n-icons".to_string(),
            get_number_of_icons!().to_string().into(),
        );
        map.insert(
            "svg".to_string(),
            format!(
                "<abbr title=\"{}\">{}</abbr>",
                tr!("scalable-vector-graphic"),
                tr!("svg"),
            )
            .into(),
        );
        map
    });

    view! {
        <div class:hidden=move || header_state().menu_open>
            <h1>"Simple Icons"</h1>
            <p id="site-description" inner_html=description_html></p>
        </div>
    }
}
