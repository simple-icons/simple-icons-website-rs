use crate::app::TITLE;
use leptos::prelude::*;
use leptos_fluent::tr;
use leptos_meta::{provide_meta_context, Link, Meta, Title};
use simple_icons_macros::get_number_of_icons;

static DOMAIN: &str = "simpleicons.org";

#[component]
pub fn Head() -> impl IntoView {
    provide_meta_context();

    let description = Signal::derive(move || {
        tr!("site-description", {
            "n-icons" => get_number_of_icons!(),
            "svg" => tr!("svg"),
        })
    });
    let url = format!("https://{}/", DOMAIN);

    view! {
        <Title text=TITLE />
        <Meta charset="utf-8" />
        <Meta content="width=device-width, initial-scale=1, shrink-to-fit=no" name="viewport" />
        <Meta name="description" content=description />
        <Link rel="apple-touch-icon" href="./apple-touch-icon.png" />
        <Link
            rel="search"
            type_="application/opensearchdescription+xml"
            title=TITLE
            href="./opensearch.xml"
        />
        <Link rel="license" href="./license.txt" />
        <Link rel="canonical" href=url />
        <MetaOpenGraph description />
        <MetaX description />
        <Meta name="msvalidate.01" content="14319924BC1F00DC15EF0EAA29E72404" />
        <Meta name="yandex-verification" content="8b467a0b98aa2725" />
        <LdJSONMetadata />
    }
}

/// Open graph meta tags
#[component]
fn MetaOpenGraph(description: Signal<String>) -> impl IntoView {
    let url = format!("https://{}/", DOMAIN);
    view! {
        <Meta name="og:type" content="website" />
        <Meta name="og:title" content=TITLE />
        <Meta name="og:description" content=description />
        <Meta name="og:url" content=url />
        <Meta name="og:site_name" content=TITLE />
        <Meta name="og:image" content="./og.png" />
    }
}

/// X (social network) meta tags
#[component]
fn MetaX(description: Signal<String>) -> impl IntoView {
    let url = format!("https://{}/", DOMAIN);
    view! {
        <Meta name="twitter:card" content="summary_large_image" />
        <Meta name="twitter:title" content=TITLE />
        <Meta name="twitter:description" content=description />
        <Meta name="twitter:url" content=url />
        <Meta name="twitter:image:src" content="./og.png" />
    }
}

/// JSON-LD metadata
/// See https://developers.google.com/search/docs/data-types/logo
#[component]
fn LdJSONMetadata() -> impl IntoView {
    let metadata = {
        let logo_url = format!("https://{}/icons/simpleicons.svg", DOMAIN);
        serde_json::json!({
            "@context": "https://schema.org",
            "@type": "Organization",
            "name": TITLE,
            "url": format!("https://{}/", DOMAIN),
            "logo": logo_url,
            "image": logo_url,
            "potentialAction": {
                "@type": "SearchAction",
                "target": format!(
                    "https://{}/?q={{search-term}}",
                    DOMAIN
                ),
                "query-input": "required name=search-term",
            },
        })
        .to_string()
    };

    view! { <script type="application/ld+json">{metadata}</script> }
}
