//! App footer

use crate::svg::SVGIcon;
use leptos::{html::Footer as FooterHtmlElement, NodeRef, *};
use leptos_fluent::I18n;
use simple_icons_macros::simple_icon_svg_path;
use std::collections::HashMap;

static X_ICON_SVG_PATH: &str = simple_icon_svg_path!("x");

/// Footer of the website
#[component]
pub fn Footer(
    /// Reference to the footer container, for using in sibling components
    container_ref: NodeRef<FooterHtmlElement>,
) -> impl IntoView {
    let i18n = expect_context::<I18n>();
    view! {
        <footer node_ref=container_ref>
            <ReportProblems/>
            <div class="flex flex-col md:flex-row justify-between">
                <About/>
                <XButton/>
            </div>
            <a
                class=concat!(
                    "w-full text-center mt-6 hover:underline focus:underline",
                    " text-[var(--link-color)] hover:text-[var(--link-color-hover)]",
                    " focus:text-[var(--link-color-hover)]",
                )

                href="https://github.com/simple-icons/simple-icons-website"
            >
                {move || i18n.tr("made-on")}
            </a>
        </footer>
    }
}

#[component]
fn ReportLink(
    /// Link URL
    href: &'static str,
    /// Link content
    children: Children,
) -> impl IntoView {
    view! {
        <a
            class=concat!(
                "text-[#00e] hover:text-[#3434ee]",
                " focus:text-[#3434ee] visited:text-[#551a8b]",
                " dark:text-[#227fff] dark:hover:text-[#3c8eff]",
                " dark:focus:text-[#3c8eff] dark:visited:text-[#a990bd]",
            )

            href=href
        >
            {children()}
        </a>
    }
}

#[component]
pub fn ReportProblems() -> impl IntoView {
    let i18n = store_value(expect_context::<I18n>());
    view! {
        <div class="flex flex-col py-8">
            <p>
                {move || i18n().tr("icon-missing")} {" "}
                <ReportLink href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=new+icon&template=icon_request.yml">
                    {move || i18n().tr("submit-a-request")}
                </ReportLink>
            </p>
            <p>
                {move || i18n().tr("icon-outdated")} {" "}
                <ReportLink href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=icon+outdated&template=icon_update.yml">
                    {move || i18n().tr("report-outdated-icon")}
                </ReportLink>
            </p>
        </div>
    }
}

#[component]
pub fn XButton() -> impl IntoView {
    let i18n = expect_context::<I18n>();
    view! {
        <a
            class="x-button"
            rel="noopener"
            role="button"
            target="_blank"
            href="https://x.com/intent/tweet?url=https://simpleicons.org&text=Simple%20Icons%3A%20free%20SVG%20icons%20for%20popular%20brands."
        >
            <SVGIcon fill="white" class="h-4 mr-3" path=X_ICON_SVG_PATH/>
            <span>{move || i18n.tr("share-this")}</span>
        </a>
    }
}

#[component]
pub fn About() -> impl IntoView {
    let i18n = store_value(expect_context::<I18n>());
    let maintained_by_html = move || {
        let i18n = i18n();
        i18n.trs("maintained-by", &{
            let mut map = HashMap::new();
            map.insert(
                "license".to_string(),
                format!(
                    "<a href=\"https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md\">{}</a>",
                    i18n.tr("cco")
                ).into(),
            );
            map.insert(
                "maintainers".to_string(),
                format!(
                    "<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                    i18n.tr("simple-icons-contributors")
                ).into(),
            );
            map
        })
    };
    let use_platform_html = move || {
        let i18n = i18n();
        i18n.trs("use-platform", &{
            let mut map = HashMap::new();
            map.insert(
                "platform".to_string(),
                format!(
                    "<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                    i18n.tr("github"),
                ).into(),
            );
            map
        })
    };
    let supported_by_html = move || {
        let i18n = i18n();
        i18n.trs("supported-by", &{
            let mut map = HashMap::new();
            map.insert(
                "platform".to_string(),
                format!(
                "<a href=\"https://opencollective.com/simple-icons\">{}</a>",
                i18n.tr("open-collective"),
            )
                .into(),
            );
            map
        })
    };
    view! {
        <div class="footer-about">
            <p inner_html=maintained_by_html></p>
            <p inner_html=use_platform_html></p>
            <p inner_html=supported_by_html></p>
        </div>
    }
}
