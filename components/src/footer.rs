//! App footer

use crate::svg_icon::SVGIcon;
use i18n::{move_tr, tr};
use leptos::{html::Footer as FooterHtmlElement, NodeRef, *};
use simple_icons_macros::simple_icon_svg_path;
use std::collections::HashMap;

static TWITTER_ICON_SVG_PATH: &str = simple_icon_svg_path!("twitter");

/// Footer of the website
#[component]
pub fn Footer(
    /// Reference to the footer container, for using in sibling components
    container_ref: NodeRef<FooterHtmlElement>,
) -> impl IntoView {
    view! {
        <footer _ref=container_ref>
            <ReportProblems/>
            <div class="flex flex-col md:flex-row justify-between">
                <About/>
                <TwitterButton/>
            </div>
            <a
                class=concat!(
                    "w-full text-center mt-6 hover:underline focus:underline",
                    " text-[var(--link-color)] hover:text-[var(--link-color-hover)]",
                    " focus:text-[var(--link-color-hover)]",
                )

                href="https://github.com/simple-icons/simple-icons-website"
            >
                {move_tr!("made-on")}
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
    view! {
        <div class="flex flex-col py-8">
            <p>
                {move_tr!("icon-missing")} {" "}
                <ReportLink href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=new+icon&template=icon_request.yml">
                    {move_tr!("submit-a-request")}
                </ReportLink>
            </p>
            <p>
                {move_tr!("icon-outdated")} {" "}
                <ReportLink href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=icon+outdated&template=icon_update.yml">
                    {move_tr!("report-outdated-icon")}
                </ReportLink>
            </p>
        </div>
    }
}

#[component]
pub fn TwitterButton() -> impl IntoView {
    view! {
        <a
            class="twitter-button"
            rel="noopener"
            role="button"
            target="_blank"
            href="https://twitter.com/intent/tweet?url=https://simpleicons.org&amp;text=Simple%20Icons%3A%20free%20SVG%20icons%20for%20popular%20brands."
        >
            <SVGIcon fill="white" class="h-4 mr-3" path=TWITTER_ICON_SVG_PATH/>
            <span>{move_tr!("share-this")}</span>
        </a>
    }
}

#[component]
pub fn About() -> impl IntoView {
    let maintained_by_html = move_tr!("maintained-by", &{
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
    });
    let use_platform_html = move_tr!("use-platform", &{
        let mut map = HashMap::new();
        map.insert(
            "platform".to_string(),
            format!(
                "<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                tr!("github"),
            ).into(),
        );
        map
    });
    let supported_by_html = move_tr!("supported-by", &{
        let mut map = HashMap::new();
        map.insert(
            "platform".to_string(),
            format!(
                "<a href=\"https://opencollective.com/simple-icons\">{}</a>",
                tr!("open-collective"),
            )
            .into(),
        );
        map
    });
    view! {
        <div class="footer-about">
            <p inner_html=maintained_by_html></p>
            <p inner_html=use_platform_html></p>
            <p inner_html=supported_by_html></p>
        </div>
    }
}
