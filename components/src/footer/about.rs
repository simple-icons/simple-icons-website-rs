use i18n::{move_tr, tr};
use leptos::*;
use std::collections::HashMap;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="footer-about">
            <p inner_html=move_tr!(
                "maintained-by",
                &{
                    let mut map = HashMap::new();
                    map.insert(
                        "license".to_string(),
                        format!(
                            "<a href=\"https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md\">{}</a>",
                            tr!("cco")
                        ).into(),
                    );
                    map.insert(
                        "maintainers".to_string(),
                        format!(
                            "<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                            tr!("simple-icons-contributors")
                        ).into(),
                    );
                    map
                }
            )></p>
            <p inner_html=move_tr!(
                "use-platform",
                &{
                    let mut map = HashMap::new();
                    map.insert(
                        "platform".to_string(),
                        format!(
                            "<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                            tr!("github"),
                        ).into(),
                    );
                    map
                }
            )></p>
            <p inner_html=move_tr!(
                "supported-by",
                &{
                    let mut map = HashMap::new();
                    map.insert(
                        "platform".to_string(),
                        format!(
                            "<a href=\"https://opencollective.com/simple-icons\">{}</a>",
                            tr!("open-collective"),
                        ).into(),
                    );
                    map
                }
            )></p>
        </div>
    }
}
