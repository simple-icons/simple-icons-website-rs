use leptos::{
    html::Footer as FooterHtmlElement,
    prelude::{NodeRef, *},
};
use leptos_fluent::{move_tr, tr};
use simple_icons_macros::get_simple_icon_svg_path;
use simple_icons_website_svg_icon::SVGIcon;

/// Footer of the website
#[component]
pub fn Footer(
    /// Reference to the footer container, for using in sibling components
    container_ref: NodeRef<FooterHtmlElement>,
) -> impl IntoView {
    view! {
        <footer node_ref=container_ref>
            <ReportProblems />
            <div class="flex-col flex md:flex-row justify-between">
                <About />
                <div class="flex flex-col mt-8 space-y-2 md:mt-auto">
                    <p class="text-center min-w-[135px]">{move_tr!("share-this")}</p>
                    <div class="flex justify-center md:justify-between space-x-2 md:space-x-0">
                        <FacebookButton />
                        <XButton />
                        <MastodonButton />
                        <BlueskyButton />
                    </div>
                </div>
            </div>
            <a
                class=concat!(
                    "w-full mt-9 text-center hover:underline focus:underline",
                    " text-[var(--link-color)] hover:text-[var(--link-color-hover)]",
                    " focus:text-[var(--link-color-hover)]",
                )

                href="https://github.com/simple-icons/simple-icons-website-rs"
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
            href=href
            class=concat!(
                "text-[#00e] hover:text-[#3434ee]",
                " focus:text-[#3434ee] visited:text-[#551a8b]",
                " dark:text-[#227fff] dark:hover:text-[#3c8eff]",
                " dark:focus:text-[#3c8eff] dark:visited:text-[#a990bd]",
            )
        >
            {children()}
        </a>
    }
}

#[component]
fn ReportProblems() -> impl IntoView {
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
fn SocialButton(
    href: &'static str,
    class: &'static str,
    title: &'static str,
    icon: &'static str,
) -> impl IntoView {
    view! {
        <a
            href=href
            title=title
            class=format!("social-button {class}")
            rel="noopener"
            role="button"
            target="_blank"
        >
            <SVGIcon width="23" height="23" path=icon />
        </a>
    }
}

#[component]
fn XButton() -> impl IntoView {
    view! {
        <SocialButton
            href="https://x.com/intent/tweet?url=https://simpleicons.org&text=Simple%20Icons%3A%20SVG%20icons%20for%20popular%20brands."
            icon=get_simple_icon_svg_path!("x")
            class="x-button"
            title="X"
        />
    }
}

#[component]
fn FacebookButton() -> impl IntoView {
    view! {
        <SocialButton
            href="https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fsimpleicons.org"
            icon=get_simple_icon_svg_path!("facebook")
            class="facebook-button"
            title="Facebook"
        />
    }
}

#[component]
fn MastodonButton() -> impl IntoView {
    view! {
        <SocialButton
            href="https://mastodonshare.com/?text=Simple%20Icons%3A%20SVG%20icons%20for%20popular%20brands.&url=https%3A%2F%2Fsimpleicons.org"
            icon=get_simple_icon_svg_path!("mastodon")
            class="mastodon-button"
            title="Mastodon"
        />
    }
}

#[component]
fn BlueskyButton() -> impl IntoView {
    view! {
        <a
            href="https://bsky.app/intent/compose?text=Simple%20Icons%3A%20SVG%20icons%20for%20popular%20brands%20at%20https%3A%2F%2Fsimpleicons.org"
            title="Bluesky"
            class="social-button bluesky-button"
            rel="noopener"
            role="button"
            target="_blank"
        >
            <svg width="23px" height="23px">
                <path d=get_simple_icon_svg_path!("bluesky") />
            </svg>
        </a>
    }
}

#[component]
fn About() -> impl IntoView {
    let maintained_by_html = move || {
        tr!("maintained-by", {
            "license" => format!(
                "<a href=\"https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md\">{}</a>",
                tr!("cco")
            ),
            "maintainers" => format!(
                "<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                tr!("simple-icons-contributors")
            ),
        })
    };
    let use_platform_html = move || {
        tr!("use-platform", {
            "platform" => format!(
                "<a href=\"https://github.com/simple-icons/simple-icons\">{}</a>",
                tr!("github"),
            )
        })
    };
    let supported_by_html = move || {
        tr!("supported-by", {
            "platform" => format!(
                "<a href=\"https://opencollective.com/simple-icons\">{}</a>",
                tr!("open-collective"),
            ),
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
