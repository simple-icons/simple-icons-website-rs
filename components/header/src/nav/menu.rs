use super::button::{HeaderMenuButton, HeaderMenuLink};
use crate::HeaderStateSignal;
use icondata::{
    AiHomeOutlined, AiMoreOutlined, RiErrorWarningSystemLine, VsPreview,
};
use leptos::{prelude::*, task::spawn_local};
use leptos_fluent::move_tr;
use leptos_use::on_click_outside;
use simple_icons_website_ids::Ids;
use web_sys_simple_fetch::fetch_text;

/// Button to show more information on the header hidden under a menu
#[component]
pub fn HeaderMenuMoreInfoButton() -> impl IntoView {
    let header_state = expect_context::<HeaderStateSignal>().0;
    let wasm_links = RwSignal::new(Vec::<String>::new());

    let menu_ref = NodeRef::new();
    let menu_open = RwSignal::new(false);
    _ = on_click_outside(menu_ref, move |_| {
        if menu_open.get_untracked() {
            menu_open.set(false);
        }
    });
    menu_ref.on_load(move |_| {
        for wasm_link in &wasm_links.get_untracked() {
            let link = wasm_link.clone();
            spawn_local(async move {
                match fetch_text(&link).await {
                    Ok(_) => {
                        #[cfg(debug_assertions)]
                        #[allow(leptos_print_stdout)]
                        {
                            leptos::logging::log!(
                                "WASM Link fetched successfully: {}",
                                link
                            );
                        }
                    }
                    Err(err) => {
                        leptos::logging::warn!(
                            "Failed to fetch WASM Link '{}': {}",
                            link,
                            err
                        );

                        #[cfg(debug_assertions)]
                        leptos::logging::warn!(concat!(
                            "[dev] Make sure that you build the",
                            " full app to turn on WASM cache optimizations.",
                        ));
                    }
                }
            });
        }
    });

    let render_links = move || {
        let build_timestamp = window()
            .document()
            .unwrap()
            .document_element()
            .unwrap()
            .get_attribute("build-timestamp")
            .unwrap_or_default();

        let links = vec![
            (move_tr!("home"), "/", AiHomeOutlined, ""),
            (
                move_tr!("preview-generator"),
                "/preview/",
                VsPreview,
                "preview",
            ),
            (
                move_tr!("deprecations"),
                "/deprecations/",
                RiErrorWarningSystemLine,
                "deprecations",
            ),
        ];
        let filtered_links = links
            .into_iter()
            .filter(|(_title, href, _icon, _wasm_slug)| {
                let location = window().location();
                let current_path = location.pathname().unwrap_or_default();
                if current_path == "/" {
                    if *href == "/" {
                        return false;
                    }
                    return true;
                }
                if *href == "/" {
                    return true;
                }
                !current_path
                    .starts_with::<&str>(href[0..href.len() - 1].as_ref())
            })
            .collect::<Vec<_>>();

        let create_base_link = |asset_slug: &str| {
            format!("/simple-icons-website{asset_slug}-{build_timestamp}")
        };

        let mut wasm_links_ = vec![];
        for (_, _, _, wasm_slug) in &filtered_links {
            let asset_slug = if wasm_slug.is_empty() {
                "".to_string()
            } else {
                format!("-{wasm_slug}")
            };

            let base_link = create_base_link(&asset_slug);
            let wasm_bg_link = format!("{base_link}_bg.wasm");
            let js_link = format!("{base_link}.js");

            wasm_links_.push(wasm_bg_link);
            wasm_links_.push(js_link);
        }

        let base_404_link = create_base_link("-404");
        let wasm_404_bg_link = format!("{base_404_link}_bg.wasm");
        let js_404_link = format!("{base_404_link}.js");
        wasm_links_.push(wasm_404_bg_link);
        wasm_links_.push(js_404_link);
        wasm_links.set(wasm_links_);

        filtered_links
            .into_iter()
            .map(|(title, href, icon, _wasm_slug)| {
                view! {
                    <HeaderMenuLink title=title href=href icon=icon blank=false>
                        {title}
                    </HeaderMenuLink>
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <li
            class:hidden=move || !header_state().menu_open
            id=Ids::More
            node_ref=menu_ref
            on:click=move |_| menu_open.update(|is_open| *is_open = !*is_open)
        >
            <HeaderMenuButton icon=AiMoreOutlined width=25 height=25 />
            <div class:hidden=move || !menu_open()>
                <ul>{render_links()}</ul>
            </div>
        </li>
    }
}
