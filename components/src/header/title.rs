use leptos::*;
use leptos_fluent::tr;
use simple_icons_macros::get_number_of_icons;

/// Header title
///
/// It includes the title and the description
/// about what is Simple Icons shown below the title.
#[component]
pub fn HeaderTitle() -> impl IntoView {
    let description_html = move || {
        tr!("site-description", {
            "n-icons" => get_number_of_icons!(),
            "svg" => format!(
                "<abbr title=\"{}\">{}</abbr>",
                tr!("scalable-vector-graphic"),
                tr!("svg"),
            ),
        })
    };

    view! {
        <div>
            <a href="/">"Simple Icons"</a>
            <p id="site-description" inner_html=description_html></p>
        </div>
    }
}
