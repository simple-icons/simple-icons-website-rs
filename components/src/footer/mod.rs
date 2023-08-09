//! App footer

mod about;

use about::*;
use i18n::move_gettext;
use leptos::{html::Footer as FooterHtmlElement, NodeRef, *};
use macros::simple_icon_svg_path;

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
                    " focus:text-[var(--link-color-hover)]"
                )
                href="https://github.com/simple-icons/simple-icons-website"
            >
                {move_gettext!("Made with ❤️ on GitHub")}
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
                "text-[#00e] hover:text-[#3434ee] focus:text-[#3434ee] visited:text-[#551a8b]",
                " dark:text-[#227fff] dark:hover:text-[#3c8eff] dark:focus:text-[#3c8eff] dark:visited:text-[#a990bd]",
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
                {move_gettext!("Icon missing?")} {" "}
                <ReportLink href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=new+icon&template=icon_request.yml">
                    {move_gettext!("Submit a request")}
                </ReportLink>
            </p>
            <p>
                {move_gettext!("Icon outdated?")} {" "}
                <ReportLink href="https://github.com/simple-icons/simple-icons/issues/new?assignees=&labels=icon+outdated&template=icon_update.yml">
                    {move_gettext!("Report outdated icon")}
                </ReportLink>
            </p>
        </div>
    }
}

#[component]
pub fn TwitterButton() -> impl IntoView {
    view! {
        <a
            class=concat!(
                "flex flex-row items-center h-0 my-auto align-middle bg-[#1DA1F2]",
                " text-white rounded-md px-3 py-5 hover:bg-[#55b8f5] focus:bg-[#55b8f5]",
                " whitespace-nowrap mx-auto md:mx-4 lg:mx-12 mt-4 md:mt-auto text-center"
            )
            rel="noopener"
            role="button"
            target="_blank"
            href="https://twitter.com/intent/tweet?url=https://simpleicons.org&amp;text=Simple%20Icons%3A%20free%20SVG%20icons%20for%20popular%20brands."
        >
            <svg fill="white" class="h-4 mr-3" role="img" viewBox="0 0 24 24">
                <path d=TWITTER_ICON_SVG_PATH></path>
            </svg>
            <span>{move_gettext!("Share this on Twitter!")}</span>
        </a>
    }
}
