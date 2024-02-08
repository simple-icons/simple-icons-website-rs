use leptos::*;
use leptos_fluent_i18n::I18n;

#[component]
pub fn IconGridItemLinks(
    /// Brand guidelines URL
    guidelines_url: Option<&'static str>,
    /// License URL
    license_url: Option<&'static str>,
    /// License type
    license_type: Option<&'static str>,
) -> impl IntoView {
    let i18n = store_value(expect_context::<I18n>());
    let brand_guidelines = move || i18n().tr("brand-guidelines");
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
                            },
                        );
                }
                if license_type.is_some() || license_url.is_some() {
                    let title = move || {
                        let i18n = i18n();
                        match license_type {
                            Some(license_type) => {
                                match license_type {
                                    "custom" => i18n.tr("custom-license"),
                                    _ => license_type.to_string(),
                                }
                            }
                            None => i18n.tr("license"),
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
                            },
                        );
                }
                links
            }

        </div>
    }
}
