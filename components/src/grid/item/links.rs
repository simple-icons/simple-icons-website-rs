use leptos::prelude::*;
use leptos_fluent::tr;

#[component]
pub fn IconGridItemLinks(
    /// Brand guidelines URL
    guidelines_url: Option<&'static str>,
    /// License URL
    license_url: Option<&'static str>,
    /// License type
    license_type: Option<&'static str>,
) -> impl IntoView {
    let brand_guidelines = move || tr!("brand-guidelines");
    view! {
        <div class="links">

            {
                let mut links = vec![];
                if let Some(guidelines_url) = guidelines_url {
                    links
                        .push(
                            view! {
                                <a
                                    href=guidelines_url
                                    title=brand_guidelines
                                    class="brand-guidelines"
                                    target="_blank"
                                >
                                    {brand_guidelines}
                                </a>
                            }
                                .into_any(),
                        );
                }
                if license_type.is_some() || license_url.is_some() {
                    let title = move || {
                        match license_type {
                            Some(license_type) => {
                                match license_type {
                                    "custom" => tr!("custom-license"),
                                    _ => license_type.to_string(),
                                }
                            }
                            None => tr!("license"),
                        }
                    };
                    links
                        .push(
                            view! {
                                <a
                                    href=match license_url {
                                        Some(license_url) => license_url.to_string(),
                                        None => {
                                            format!(
                                                "https://spdx.org/licenses/{}",
                                                license_type.unwrap(),
                                            )
                                        }
                                    }

                                    title=title
                                    class="license"
                                    target="_blank"
                                >
                                    {title}
                                </a>
                            }
                                .into_any(),
                        );
                }
                links
            }

        </div>
    }
}
